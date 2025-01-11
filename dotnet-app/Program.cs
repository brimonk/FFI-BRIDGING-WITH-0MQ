using System;

// Brian Chrzanowski
//
// What we're going to do in this project is a little unorthodox; however, it's certainly something
// that we need. In theory, what we want is like, local RPC using ZeroMQ. Currently, there are
// some small limitations to ZeroMQ's Windows capabilities. The kind of big one is there isn't
// 'ipc' support on Windows. In a perfect world we would have this support from Windows named pipes.
//
// Given that we don't have this currently, my plan is to communicate cross-language via 0MQ context
// sharing. So, at boot-up, we have some function that just returns the `void *` that the context
// is to dotnet, and at shutdown either we leak this from the dotnet side, or something else.
//
// TODO (Brian)
// - Message Serialization / Deserialization via 'SimpleMsgPack'
// - Expose 0MQ Context at Library Startup (rust-native-lib)
// - Bind a req/resp socket from dotnet
// - Use it
// - confirm that the entire client <-> client lib <-> server flow works

namespace MyApp
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello World!");
        }
    }
}
