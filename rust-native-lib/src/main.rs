use rmp_serde::Serializer;
use serde::Serialize;

fn main() {
    println!("Connecting to hello world server...");
    let context = zmq::Context::new();

    let requester = context.socket(zmq::REQ).unwrap();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.connect("tcp://localhost:5500").is_ok());
    assert!(requester.connect("tcp://localhost:5555").is_ok());

    loop {
        
    }

    for reqnum in 0..10 {
        let request = common::ProduceRequest {
            key: None,
            topic: "TESTING-TOPIC".into(),
            partition: -1,
        };

        let mut req = Vec::new();
        request.serialize(&mut Serializer::new(&mut req)).unwrap();

        println!("Sending Produce Request {:?}...", reqnum);
        requester.send(req, 0).unwrap();

        let resp = requester
            .recv_bytes(0)
            .expect("Couldn't receive a message!");

        let response = rmp_serde::from_slice::<common::ProduceResponse>(resp.as_slice()).unwrap();

        if response.result == 0 {
            println!("Produce was OK.");
        } else {
            println!("Produce was NOT OK.");
        }
    }
}
