use bencoding::{Value, to_value};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Request {
    pub info_hash: String,
    pub peer_id: String,
    pub ip: Option<String>,
    pub port: i64,
    pub uploaded: i64,
    pub downloaded: i64,
    pub left: i64,
    pub event: String,
}

impl From<&Request> for Value {
    fn from(value: &Request) -> Self {
        to_value(value)
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Peer {
    pub peer_id: String,
    pub ip: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SuccessRespone {
    pub interval: i64,
    pub peers: Vec<Peer>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct FailureResponse {
    pub failure_reason: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub enum Response {
    Success(SuccessRespone),
    Failure(FailureResponse),
}

impl From<&Response> for Value {
    fn from(value: &Response) -> Self {
        to_value(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happypath_bencoded_request() -> Result<(), Box<dyn std::error::Error>> {
        let _ = Value::from(&Request {
            info_hash: String::from(""),
            peer_id: String::from(""),
            ip: Some(String::from("")),
            port: 0,
            uploaded: 0,
            downloaded: 0,
            left: 0,
            event: String::from(""),
        });

        Ok(())
    }

    #[test]
    fn happypath_bencoded_success_response() -> Result<(), Box<dyn std::error::Error>> {
        let _ = Value::from(&Response::Success(SuccessRespone {
            interval: 0,
            peers: vec![Peer {
                peer_id: String::from(""),
                ip: Some(String::from("")),
            }],
        }));

        Ok(())
    }

    #[test]
    fn happypath_bencoded_failure_response() -> Result<(), Box<dyn std::error::Error>> {
        let _ = Value::from(&Response::Failure(FailureResponse {
            failure_reason: String::from(""),
        }));

        Ok(())
    }
}
