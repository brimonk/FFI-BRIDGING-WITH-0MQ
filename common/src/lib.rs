use serde::{Deserialize, Serialize};

pub enum Request {
    Produce(ProduceRequest),
}

pub enum Response {
    Produce(ProduceResponse),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ProduceRequest {
    pub key: Option<String>,
    pub topic: String,
    pub partition: i32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ProduceResponse {
    pub result: i32,
}
