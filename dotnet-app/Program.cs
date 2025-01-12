using System;
using MessagePack;
using ZeroMQ;

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

namespace ExampleDotnetApp
{
    internal class Program
    {
        static void Main(string[] args)
        {
            using (var socket = new ZSocket(ZSocketType.REQ))
            {
                socket.Connect("tcp://localhost:5500");

                for (int i = 0; i < 10; i++)
                {
                    var request = new ProduceRequest("TOPIC-GOES-HERE", 4, null);
                    byte[] request_buffer = MessagePackSerializer.Serialize(request);

                    var message = new ZMessage(new List<ZFrame>() { new ZFrame(request_buffer) });

                    socket.Send(message);

                    var response_message = socket.ReceiveMessage();
                    var response_buffer = new List<byte>();

                    foreach (var frame in response_message.AsEnumerable())
                    {
                        response_buffer.AddRange(frame.Read());
                    }

                    var response = MessagePackSerializer.Deserialize<ProduceResponse>(response_buffer.ToArray());

                }
            }
        }
    }

    [MessagePackObject]
    public class ProduceRequest
    {
        [Key(0)]
        public string? key;

        [Key(1)]
        public string topic;

        [Key(2)]
        public int partition;

        public ProduceRequest(string topic, int partition, string? key)
        {
            this.key = key;
            this.topic = topic;
            this.partition = partition;
        }
    }

    [MessagePackObject]
    public class ProduceResponse
    {
        [Key(0)]
        public int result;
    }
}
