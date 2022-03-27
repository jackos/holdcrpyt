use tracing::{info, error};
use lambda_http::{Error, IntoResponse,  Body, Response};
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Res {
    #[serde(skip_serializing)]
   status: u16,
   message: String,
   #[serde(skip_serializing_if = "String::is_empty")]
   error: String,
}

impl Res {
    pub fn ok(message: &str) -> Res {
        Res { status: 200, message: message.to_string(), error: String::new()}
    }

    pub fn parse_error(error: serde_json::Error) -> Res {
        Res { status: 400, message: "failed to parse json body".to_string(), error: error.to_string()}
    }

    pub fn bad_request(message: &str) -> Res {
        Res { status: 400, message: message.to_string(), error: String::new() }
    }

    pub fn internal_server_error(message: &str, error: Error) -> Res {
        Res { status: 500,  message: message.to_string(), error: error.to_string(), }
    }
}

impl IntoResponse for Res {
    fn into_response(self) -> Response<Body> {
        info!(r#"{{"status_code": {}}}"#, self.status);
        let body = serde_json::to_string(&self).unwrap();
        if self.status < 400 {
           info!("{}", body) 
        } else {
           error!("{}", body) 
        }
        Response::builder()
            .status(self.status)
            .body(Body::Text(body))
            .expect("unable to build http::Response")
    }
}
