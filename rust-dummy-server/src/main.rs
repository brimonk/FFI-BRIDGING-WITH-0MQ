use common::ProduceResponse;
use rmp_serde::Serializer;
use serde::Serialize;

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    loop {
        let request_buffer = responder.recv_bytes(0).unwrap();
        let request =
            rmp_serde::from_slice::<common::ProduceRequest>(request_buffer.as_slice()).unwrap();

        println!("REQUEST:");
        println!("  TOPIC: {}", request.topic);
        println!("  KEY: {:?}", request.key);
        println!("  PARTITION: {}", request.partition);

        // do real work here...

        let response = ProduceResponse {
            result: 0
        };

        let mut response_buffer = Vec::new();
        response.serialize(&mut Serializer::new(&mut response_buffer)).unwrap();

        responder.send(response_buffer, 0).unwrap();
    }
}
