use std::{sync::OnceLock, thread, time::Duration};

use zmq::Message;

static ZMQ_CONTEXT: OnceLock<zmq::Context> = OnceLock::new();

fn client(endpoint: String) {
    let context = ZMQ_CONTEXT.get_or_init(|| {
        zmq::Context::new()
    });

    thread::sleep(Duration::from_secs(2));

    let socket = context.socket(zmq::REQ).unwrap();
    socket.connect(endpoint.as_str()).unwrap();

    for i in 0..10 {
        let s = format!("Hello World! {i}");
        socket.send(&s, 0).unwrap();
        let mut message: Message = Message::new();
        socket.recv(&mut message, 0).unwrap();
    }
}

fn main() {
    let version = zmq::version();

    println!("Version: {}.{}.{}", version.0, version.1, version.2);

    let context = ZMQ_CONTEXT.get_or_init(|| {
        zmq::Context::new()
    });

    let socket = context.socket(zmq::REP).unwrap();
    // socket.bind(&IPC_REFERENCE).unwrap();
    socket.bind("ipc://*").unwrap();

    let endpoint = socket.get_last_endpoint().unwrap().unwrap();
    println!("ENDPOINT: {}", endpoint);

    let join = std::thread::spawn(|| { client(endpoint); });

    for _ in 0..10 {
        let msg = socket.recv_bytes(0).unwrap();
        let str = String::from_utf8(msg).unwrap();
        println!("Got message: {}", str);
        socket.send("", 0).unwrap();
    }

    join.join().unwrap();
}
