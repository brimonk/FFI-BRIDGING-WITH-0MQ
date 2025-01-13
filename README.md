# FFI <-> 0MQ Bridge

This small repo purely exists as an example for how we might use ZeroMQ to hop FFI boundaries and
not eat a ton of code time, code gen, API surface area on crossing this gap. Given that ZeroMQ is
really just a generic message queueing library that has some niceties around a generic socket
connection, in the future if we wanted to, it'd be trivial to swap the client library for a
"client server" or something similar. How would we do this? We swap the "ipc://" protocol for
"tcp://", and it's done.

## Examples? Where?

There's a Rust 'server' that will create and bind to an `IPC` socket at `\\TESTING-IPC.ipc`. As of
ZeroMQ v4.3.3 (find the link to the PR in a commit to this repo), the library supports IPC on
Windows given that Windows actually supports AF\_UNIX domain sockets just as Linux does.

You can do a `cargo r` on `test-ipc`, and perform a `dotnet run` from within `dotnet-app` to see
this example working.

## Problems

There are a few problems with this solution, and it comes down to the maintainership of this
`clrzmq4` library. There is an alternative that implements the ZMQ protocol / contract entirely in
C#; however, it doesn't have this IPC support.

Additionally, the version that exists on nuget.org doesn't have the correct version of libzmq
either. So, to truly get this to work, I had to compile libzmq myself, then include that in the
build for the dotnet-app.

Additionally, I haven't audited the security piece of the ZeroMQ stuff, so how useful it is
alongside other security constraints, I do not know.
