//! This is a helper struct for returning responses when using lambda_http
//! it implements `IntoResponse` which creates a response in the desired format
//! so that we can satisfy the Result<impl IntoResponse, Error> signature.
//! It has different helper methods to make it easy to return responses with a 
//! provided signature. It also logs the responses using the `tracing` crate.
//! 
//! Different response types will return different information in the body e.g. errors
//! will contain an `"error"` key with the returned error that caused the lambda to fail.
//! 
//! For methods with a body, it overwrites the other fields, commonly used to return a map or struct
use tracing::{info, error};
use lambda_http::{Error, IntoResponse,  Body, Response};
use serde::{Serialize};

#[derive(Serialize, Debug, Default)]
pub struct Res {
    #[serde(skip_serializing)]
   status: u16,
   message: String,
   #[serde(skip_serializing_if = "String::is_empty")]
   error: String,
   #[serde(skip_serializing_if = "String::is_empty")]
   body: String,
}

impl Res {
    pub fn ok(message: &str) -> Res {
        Res { status: 200, message: message.to_string(), ..Default::default() }
    }

    pub fn ok_body(body: &str) -> Res {
        Res { status: 200, body: body.to_string(), ..Default::default() }
    }

    pub fn parse_body_error(error: serde_json::Error) -> Res {
        Res { status: 400, message: "failed to parse json body".to_string(), error: error.to_string(), ..Default::default()}
    }

    pub fn parse_response_error(message: &str, error: reqwest::Error) -> Res {
        Res { status: 400, message: message.to_string(), error: error.to_string(), ..Default::default()}
    }

    pub fn bad_request(message: &str) -> Res {
        Res { status: 400, message: message.to_string(), ..Default::default() }
    }

    pub fn internal_server_error(message: &str, error: Error) -> Res {
        Res { status: 500,  message: message.to_string(), error: error.to_string(), ..Default::default() }
    }
}


impl IntoResponse for Res {
    fn into_response(self) -> Response<Body> {
        let body: String;
        if self.body != "" {
            body = self.body
        } else {
            body = serde_json::to_string(&self).unwrap();
        }
        info!(r#"{{"status_code": {}}}"#, self.status);
        if self.status < 400 {
            info!("{}", body) 
        } else {
            error!("{}", body) 
        }

        Response::builder()
            .header("Access-Control-Allow-Origin", "https://holdcrypt.com")
            .header("Access-Control-Allow-Methods", "*")
            .header("Access-Control-Allow-Headers", "*")
            .status(self.status)
            .body(Body::Text(body))
            .expect("unable to build http::Response")
    }
}
