namespace Wasi.Experimental.Http
{
    using System;
    using System.Collections.Generic;
    using System.Globalization;
    using System.IO;
    using System.Linq;
    using System.Net.Http;
    using System.Net.Http.Headers;
    using System.Text;
    using System.Threading;
    using Microsoft.Extensions.Logging;
    using Wasi.Experimental.Http.Exceptions;
    using Wasmtime;

    /// <summary>
    /// HttpRequestHandler provides support for wasi_experimental_http.
    /// </summary>
    internal class HttpRequestHandler : IDisposable
    {
        /// <summary>
        /// DefaultHttpRequestLimit specifies the default HTTP Request Limit for a module.
        /// </summary>
        public const int DefaultHttpRequestLimit = 10;

        /// <summary>
        /// MaxHttpRequestLimit specifies the maximum HTTP Request Limit for a module.
        /// </summary>
        public const int MaxHttpRequestLimit = 500;
        private const string ModuleName = "wasi_experimental_http";
        private const string MemoryName = "memory";
        private const int OK = 0;
        private const int RuntimeError = 12;

        private readonly Dictionary<int, Response> responses;
        private readonly string[] allowedMethods = new string[] { "DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT", "TRACE" };
        private readonly ILogger logger;
        private readonly HttpClient httpClient;
        private readonly List<Uri> allowedHosts;
        private readonly int maxHttpRequests;

        private int lastResponse;

        /// <summary>
        /// Initializes a new instance of the <see cref="HttpRequestHandler"/> class.
        /// </summary>
        /// <param name="linker">The Wasmtime linker.</param>
        /// <param name="store">The Wasmtime store.</param>
        /// <param name="loggerFactory">ILoggerFactory.</param>
        /// <param name="httpClientFactory">IHttpClientFactory to be used for module Http Requests. </param>
        /// <param name="allowedHosts">A set of allowedHosts (hostnames) that the module can send HTTP requests to.</param>
        /// <param name="maxHttpRequests">The maximum number of requests that can be made by a module.</param>
        public HttpRequestHandler(Linker linker, Store store, ILoggerFactory loggerFactory, IHttpClientFactory httpClientFactory, int maxHttpRequests, List<Uri> allowedHosts = null)
        {
            this.logger = loggerFactory.CreateLogger(typeof(HttpRequestHandler).FullName);
            this.httpClient = httpClientFactory.CreateClient();
            this.allowedHosts = allowedHosts;
            this.maxHttpRequests = maxHttpRequests;
            this.responses = new Dictionary<int, Response>();

            linker.Define(ModuleName, "body_read", Function.FromCallback<Caller, int, int, int, int, int>(store, this.ReadBody));
            linker.Define(ModuleName, "close", Function.FromCallback<Caller, int, int>(store, this.Close));
            linker.Define(ModuleName, "req", Function.FromCallback<Caller, int, int, int, int, int, int, int, int, int, int, int>(store, this.Request));
            linker.Define(ModuleName, "header_get", Function.FromCallback<Caller, int, int, int, int, int, int, int>(store, this.GetHeader));
            linker.Define(ModuleName, "headers_get_all", Function.FromCallback<Caller, int, int, int, int, int>(store, this.GetAllHeaders));
        }

        public void Dispose()
        {
            this.Dispose(true);
            GC.SuppressFinalize(this);
        }

        protected virtual void Dispose(bool disposing)
        {
            if (disposing)
            {
                this.httpClient?.Dispose();
                foreach (var response in this.responses)
                {
                    response.Value.Dispose();
                }
            }
        }

        private static Memory GetMemory(Caller caller)
        {
            var memory = caller.GetMemory(MemoryName);
            if (memory is null)
            {
                throw new MemoryNotFoundException();
            }

            return memory;
        }

        private int ReadBody(Caller caller, int handle, int bufferPtr, int bufferLength, int bufferWrittenPtr)
        {
            this.logger.LogTrace($"ReadBody called with handle {handle}");
            try
            {
                var memory = GetMemory(caller);
                var response = this.GetResponse(handle);
                var available = Math.Min(Convert.ToInt32(response.Content.Length) - Convert.ToInt32(response.Content.Position), bufferLength);
                response.Content.Read(memory.GetSpan(caller).Slice(bufferPtr, available));
                memory.WriteInt32(caller, bufferWrittenPtr, available);
                return OK;
            }
            catch (ExperimentalHttpException ex)
            {
                return ex.ErrorCode;
            }
#pragma warning disable CA1031
            catch (Exception ex)
#pragma warning restore CA1031
            {
                this.logger.LogTrace($"Exception: {ex}");
                return RuntimeError;
            }
        }

        private Response GetResponse(int handle)
        {
            var response = this.responses[handle];
            if (response == null)
            {
                this.logger.LogTrace($"Failed to get response Handle: {handle}");
                throw new InvalidHandleException();
            }

            return response;
        }

        private int Close(Caller call, int handle)
        {
            {
                this.logger.LogTrace($"Function close was called  with handle {handle}");
                try
                {
                    var response = this.GetResponse(handle);
                    this.responses.Remove(handle);
                    response.Dispose();
                    return OK;
                }
                catch (ExperimentalHttpException ex)
                {
                    return ex.ErrorCode;
                }
#pragma warning disable CA1031
                catch (Exception ex)
#pragma warning restore CA1031
                {
                    this.logger.LogTrace($"Exception: {ex}");
                    return RuntimeError;
                }
            }
        }

        private int Request(Caller caller, int urlPtr, int urlLength, int methodPtr, int methodLength, int headersPtr, int headersLength, int bodyPtr, int bodyLength, int statusCodePtr, int handlePtr)
        {
            this.logger.LogTrace("Function req was called");
            try
            {
                var memory = GetMemory(caller);
                var url = this.ValidateHostAllowed(caller, memory, urlPtr, urlLength);
                var method = this.ValidateMethod(caller, memory, methodPtr, methodLength);
                var headers = this.GetHttpRequestHeaders(caller, memory, headersPtr, headersLength);
                var body = this.GetRequestBody(caller, memory, bodyPtr, bodyLength);
                var httpResponseMessage = this.SendHttpRequest(url, method, headers, body);
                memory.WriteInt32(caller, statusCodePtr, (int)httpResponseMessage.StatusCode);
                var handle = Interlocked.Increment(ref this.lastResponse);
                if (handle > this.maxHttpRequests)
                {
                    throw new TooManySessionsException();
                }

                var response = new Response(httpResponseMessage);
                this.responses.Add(handle, response);
                memory.WriteInt32(caller, handlePtr, handle);
                this.logger.LogTrace($"Function req created handle {handle}");
                return OK;
            }
            catch (ExperimentalHttpException ex)
            {
                return ex.ErrorCode;
            }
#pragma warning disable CA1031
            catch (Exception ex)
#pragma warning restore CA1031
            {
                this.logger.LogTrace($"Exception: {ex}");
                return RuntimeError;
            }
        }

        private int GetHeader(Caller caller, int handle, int namePtr, int nameLength, int valuePtr, int valueLength, int valueWrittenPtr)
        {
            this.logger.LogTrace($"Function header_get was called with handle {handle}");
            try
            {
                var memory = GetMemory(caller);
                string headerName;
                try
                {
                    headerName = memory.ReadString(caller, namePtr, nameLength);
                }
                catch (Exception ex)
                {
                    var message = $"Failed to read header  Exception: {ex.Message}";
                    this.logger.LogTrace(message);
                    throw new MemoryAccessException(message, ex);
                }

                this.logger.LogTrace($"header_get Header Name: {headerName}");
                var response = this.GetResponse(handle);

                HttpHeaders headers;
                if (headerName.StartsWith("content", true, CultureInfo.InvariantCulture))
                {
                    headers = response.HttpResponseMessage.Content.Headers;
                }
                else
                {
                    headers = response.HttpResponseMessage.Headers;
                }

                var headerValues = headers.Where(h => h.Key.ToUpperInvariant() == headerName.ToUpperInvariant()).Select(h => h.Value).FirstOrDefault();
                if (headerValues == null)
                {
                    this.logger.LogTrace($"Failed to get Header {headerName}");
                }

                var headerValue = string.Join(';', headerValues);
                var headerValueLength = headerValue.Length;
                if (headerValueLength > valueLength)
                {
                    var message = $"Header Value for {headerName} Too Big. Bufffer Length {valueLength} Header Length {headerValueLength}";
                    this.logger.LogTrace(message);
                    throw new BufferTooSmallException(message);
                }

                memory.WriteString(caller, valuePtr, headerValue);
                memory.WriteInt32(caller, valueWrittenPtr, headerValueLength);
                return OK;
            }
            catch (ExperimentalHttpException ex)
            {
                return ex.ErrorCode;
            }
#pragma warning disable CA1031
            catch (Exception ex)
#pragma warning restore CA1031
            {
                this.logger.LogTrace($"Exception: {ex}");
                return RuntimeError;
            }
        }

        private int GetAllHeaders(Caller caller, int handle, int bufferPtr, int bufferLength, int bufferWrittenPtr)
        {
            this.logger.LogTrace($"Function headers_get_all was called with handle {handle}");
            try
            {
                var memory = GetMemory(caller);
                var response = this.GetResponse(handle);
                var allHeaders = new StringBuilder();

                foreach (var header in response.HttpResponseMessage.Headers)
                {
                    this.logger.LogTrace($"Getting Response Header: {header.Key}");
                    allHeaders.AppendLine($"{header.Key}:{string.Join(';', header.Value)}");
                }

                foreach (var header in response.HttpResponseMessage.Content.Headers)
                {
                    this.logger.LogTrace($"Getting Content Header: {header.Key}");
                    allHeaders.AppendLine($"{header.Key}:{string.Join(';', header.Value)}");
                }

                var headerValuesLength = allHeaders.Length;
                if (headerValuesLength > bufferLength)
                {
                    var message = $"Header Values for all header Too Big. Bufffer Length {bufferLength} Header Length {headerValuesLength}";
                    this.logger.LogTrace(message);
                    throw new BufferTooSmallException(message);
                }

                memory.WriteString(caller, bufferPtr, allHeaders.ToString());
                memory.WriteInt32(caller, bufferWrittenPtr, headerValuesLength);
                return OK;
            }
            catch (ExperimentalHttpException ex)
            {
                return ex.ErrorCode;
            }
#pragma warning disable CA1031
            catch (Exception ex)
#pragma warning restore CA1031
            {
                this.logger.LogTrace($"Exception: {ex}");
                return RuntimeError;
            }
        }

        private string ValidateHostAllowed(Caller caller, Memory memory, int urlPtr, int urlLength)
        {
            string url;
            try
            {
                url = memory.ReadString(caller, urlPtr, urlLength);
            }
            catch (Exception ex)
            {
                var message = $"Failed to read url Exception: {ex.Message}";
                this.logger.LogTrace(message);
                throw new MemoryAccessException(message, ex);
            }

            if (string.IsNullOrEmpty(url))
            {
                this.logger.LogTrace("Request Url is missing");
                throw new InvalidUrlException();
            }

            this.logger.LogTrace($"Request URL: {url}");

            if (!Uri.TryCreate(url, UriKind.Absolute, out var uri))
            {
                this.logger.LogTrace($"Url {url} is invalid");
                throw new InvalidUrlException();
            }

            if (this.allowedHosts != null && !this.allowedHosts.Select(a => a.Host.ToUpperInvariant() == uri.Host.ToUpperInvariant()).Any())
            {
                this.logger.LogTrace($"host {uri.Host} not allowed");
                throw new DestinationNotAllowedException();
            }

            return url;
        }

        private string ValidateMethod(Caller caller, Memory memory, int methodPtr, int methodLength)
        {
            string method;
            try
            {
                method = memory.ReadString(caller, methodPtr, methodLength);
            }
            catch (Exception ex)
            {
                var message = $"Failed to read method Exception: {ex.Message}";
                this.logger.LogTrace(message);
                throw new MemoryAccessException(message, ex);
            }

            if (string.IsNullOrEmpty(method))
            {
                this.logger.LogTrace("Request Method is missing");
                throw new InvalidMethodException();
            }

            if (!this.allowedMethods.Contains(method.ToUpperInvariant()))
            {
                this.logger.LogTrace($"Request Method {method} is not allowed");
                throw new InvalidMethodException();
            }

            this.logger.LogTrace($"Request Method: {method}");
            return method;
        }

        private Dictionary<string, string> GetHttpRequestHeaders(Caller caller, Memory memory, int headersPtr, int headersLength)
        {
            var headers = new Dictionary<string, string>();
            string headersAsString;
            try
            {
                headersAsString = memory.ReadString(caller, headersPtr, headersLength);
            }
            catch (Exception ex)
            {
                var message = $"Failed to read headers Exception: {ex.Message}";
                this.logger.LogTrace(message);
                throw new MemoryAccessException(message, ex);
            }

            if (string.IsNullOrEmpty(headersAsString))
            {
                this.logger.LogTrace($"No Request Headers Provided");
                return headers;
            }

            using var stringReader = new StringReader(headersAsString);
            var line = string.Empty;
            while ((line = stringReader.ReadLine()) != null)
            {
                var index = line.IndexOf(':', StringComparison.InvariantCultureIgnoreCase);
                var name = line.Substring(0, index);
                var value = line[++index..];
                this.logger.LogTrace($"Adding Header {name}");
                headers.Add(name, value);
            }

            return headers;
        }

        private byte[] GetRequestBody(Caller caller, Memory memory, int bodyPtr, int bodyLength)
        {
            byte[] body;
            try
            {
                body = memory.GetSpan(caller).Slice(bodyPtr, bodyLength).ToArray();
            }
            catch (Exception ex)
            {
                var message = $"Failed to get request body Exception: {ex.Message}";
                this.logger.LogTrace(message);
                throw new MemoryAccessException(message, ex);
            }

            return body;
        }

        private HttpResponseMessage SendHttpRequest(string url, string method, Dictionary<string, string> headers, byte[] body = null)
        {
            HttpResponseMessage httpResponseMessage = null;
            var httpMethod = new HttpMethod(method);
            using var req = new HttpRequestMessage(httpMethod, url);
            if (body != null && body.Length > 0)
            {
                req.Content = new ByteArrayContent(body);
            }

            logger.LogTrace($"Body: {System.Text.Encoding.Default.GetString(body)}");
            

            var contentHeaders = headers.Where(h => h.Key.StartsWith("CONTENT", true, CultureInfo.InvariantCulture));

            foreach (var contentHeader in contentHeaders)
            {
                this.logger.LogTrace($"Adding content header key {contentHeader.Key} value {contentHeader.Value}");
                req.Content?.Headers.Add(contentHeader.Key, contentHeader.Value);
                headers.Remove(contentHeader.Key);
            }

            if (!req.Content?.Headers.Contains("Content-Type") ?? false) {
                req.Content?.Headers.Add("Content-Type", "application/json");
            }

            foreach (var header in headers)
            {
                try
                {
                    this.logger.LogTrace($"Adding header key {header.Key}");
                    req.Headers.Add(header.Key, header.Value.Split(';'));
                }
                catch (Exception ex)
                {
                    var message = $"Failed to add HTTP Header {header.Key} Exception: {ex.Message}";
                    this.logger.LogTrace(message);
                    throw new InvalidEncodingException(message, ex);
                }
            }

            try
            {
                httpResponseMessage = this.httpClient.Send(req);
            }
            catch (Exception ex)
            {
                var message = $"Failed to make HTTP Request Exception: {ex.Message}";
                this.logger.LogTrace(message);
                throw new RequestException(message, ex);
            }

            return httpResponseMessage;
        }
    }
}