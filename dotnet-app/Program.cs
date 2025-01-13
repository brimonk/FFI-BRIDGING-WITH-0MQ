using System;
using MessagePack;
using ZeroMQ;

namespace ExampleDotnetApp
{
    internal class Program
    {
        static void Main(string[] args)
        {
            using (var socket = new ZSocket(ZSocketType.REQ))
            {
                socket.Connect("ipc://\\TESTING-IPC.ipc");

                for (int i = 0; i < 10; i++)
                {
                    string request = $"Hello World! {i}";
                    Console.WriteLine($"Sending: {request}");

                    var message = new ZMessage();
                    message.Add(new ZFrame(request));

                    socket.SendMessage(message);

                    Console.WriteLine("Receiving a message!");

                    var response_message = socket.ReceiveMessage();
                    byte[] response_bytes = response_message[0].Read();

                    string response = System.Text.Encoding.UTF8.GetString(response_bytes, 0, response_bytes.Length);

                    Console.WriteLine($"Received: {response}");
                }
            }
        }

        void JunkFunction()
        {
            #if false
            // var request = new ProduceRequest("TOPIC-GOES-HERE", 4, null);
            // byte[] request_buffer = MessagePackSerializer.Serialize(request);

            var request = $"Hello World {i}";
            socket.Send(request);

            byte[] request_buffer = System.Text.Encoding.ASCII.GetBytes(request);

            var message = new ZMessage(new List<ZFrame>() { new ZFrame(request_buffer) });

            socket.Send(message);

            var response_message = socket.ReceiveMessage();
            // var response_buffer = new List<byte>();

            // foreach (var frame in response_message.AsEnumerable())
            // {
            //     response_buffer.AddRange(frame.Read());
            // }

            // var response = MessagePackSerializer.Deserialize<ProduceResponse>(response_buffer.ToArray());
            #endif
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
