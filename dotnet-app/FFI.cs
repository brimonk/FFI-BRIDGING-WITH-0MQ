using System;
using System.Runtime.InteropServices;

namespace ExampleDotnetApp
{
    internal class FFI
    {
        // NOTE this is a funny path because of our funny directory structure
        [DllImport(@"../target/debug/rust_native_lib.dll")]
        internal static extern IntPtr get_zmq_context_pointer();
    }

    class Producer
    {


        public Producer()
        {
        }
    }
}
