using System;
using Wasmtime;

namespace testhost
{
    class Program
    {
        static void Main()
        {
            using var engine = new Engine();

            using var module = Module.FromTextFile(engine, "rpaas-ext-sample.wat");
            var config = new WasiConfiguration().WithInheritedStandardInput().WithInheritedStandardOutput();
            using var linker = new Linker(engine);
            using var store = new Store(engine);
            store.SetWasiConfiguration(config);
            linker.DefineWasi();
            linker.Define("rpaas_host", "TraceInfo", Function.FromCallback<Caller, int, int>(store, TraceInfo));
            linker.Define("rpaas_host", "TraceError", Function.FromCallback<Caller, int, int>(store, TraceError));
            linker.Define("rpaas_host", "TraceWarning", Function.FromCallback<Caller, int, int>(store, TraceWarning));
            var instance = linker.Instantiate(store, module);
            var validateCreate = instance.GetFunction(store, "ResourceCreationValidate");
            validateCreate?.Invoke(store);
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