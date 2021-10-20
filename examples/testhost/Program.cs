using System;
using System.Net.Http;
using Wasmtime;
using Wasi.Experimental.Http;
using Microsoft.Extensions.Http;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.DependencyInjection;
using System.Collections.Generic;
using System.IO;

namespace testhost
{
    class Program
    {
        static void Main()
        {
           
            using var engine = new Engine();
            var args=Environment.GetCommandLineArgs();
            Module getModule()
            {
                var moduleName = args.Length > 2 ? args[2] : "rpaas-ext-sample.wat";
                if (moduleName.ToLowerInvariant().EndsWith(".wat"))
                {
                    return Module.FromTextFile(engine, moduleName);
                }
                return Module.FromFile(engine, moduleName);
            }

            var entrypoint = args.Length > 1 ? args[1] : "ResourceCreationValidate";
            using var module = getModule();

            var config = new WasiConfiguration().WithInheritedStandardInput().WithInheritedStandardOutput().WithInheritedEnvironment();
            using var linker = new Linker(engine);
            using var store = new Store(engine);
            store.SetWasiConfiguration(config);
            linker.DefineWasi();
            linker.Define("rpaas_host", "TraceInfo", Function.FromCallback<Caller, int, int>(store, TraceInfo));
            linker.Define("rpaas_host", "TraceError", Function.FromCallback<Caller, int, int>(store, TraceError));
            linker.Define("rpaas_host", "TraceWarning", Function.FromCallback<Caller, int, int>(store, TraceWarning));
            
            var loggerFactory =  LoggerFactory.Create(builder => {
                builder
                .AddFilter("*",LogLevel.Trace)
                .AddConsole();
            });

            var serviceProvider = new ServiceCollection().AddHttpClient().BuildServiceProvider();
            var httpClientFactory = serviceProvider.GetService<IHttpClientFactory>();
            var hosts = new List<Uri>{new Uri("https://management.azure.com")};
            var wasiExperimentalHttpHandler = new HttpRequestHandler(linker, store, loggerFactory, httpClientFactory, 100, hosts);
            var instance = linker.Instantiate(store, module);
            var function = instance.GetFunction(store, entrypoint);
            if (function == null)
            {
                Console.WriteLine($"WASM Entrypoint {entrypoint} not found");
                return;
            }
            function?.Invoke(store);
        }
        static void TraceInfo(Caller caller, int addr, int len)
        {
            Console.WriteLine($"Trace: {caller.GetMemory("memory").ReadString(caller, addr, len)}");
        }
        static void TraceError(Caller caller, int addr, int len)
        {
            Console.WriteLine($"Error: {caller.GetMemory("memory").ReadString(caller, addr, len)}");
        }
        static void TraceWarning(Caller caller, int addr, int len)
        {
            Console.WriteLine($"Warning: {caller.GetMemory("memory").ReadString(caller, addr, len)}");
        }
    }
}