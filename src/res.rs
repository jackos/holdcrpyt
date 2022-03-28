//! Contains a struct and helper methods for returning responses when using lambda_http.
//! it implements `IntoResponse` which creates a response in the desired format
//! so that we can satisfy the Result<impl IntoResponse, Error> signature.
//! It has different helper methods to make it easy to return responses with a
//! provided signature. It also logs the responses using the `tracing` crate.
//!
//! Different response types will return different information in the body e.g. errors
//! will contain an `"error"` key with the returned error that caused the lambda to fail.
//!
//! For methods with a body, it overwrites the other fields, used to return a map or struct
//! after serializing it
use lambda_http::{Body, Error, IntoResponse, Response};
use serde::Serialize;
use tracing::{error, info};

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
    /// Returns a json body with a single key: `message`
    pub fn ok(message: &str) -> Res {
        Res {
            status: 200,
            message: message.to_string(),
            ..Default::default()
        }
    }

    /// Returns a custom json body, serialize to a &str before passing in
    pub fn ok_body(body: &str) -> Res {
        Res {
            status: 200,
            body: body.to_string(),
            ..Default::default()
        }
    }

    /// when failing to parse json from body, returns the error in the `error` key
    pub fn parse_body_error(error: serde_json::Error) -> Res {
        Res {
            status: 400,
            message: "failed to parse json body".to_string(),
            error: error.to_string(),
            ..Default::default()
        }
    }

    /// when the user has passed bad path params, query params or body, use the message to explain
    /// what they did wrong
    pub fn bad_request(message: &str) -> Res {
        Res {
            status: 400,
            message: message.to_string(),
            ..Default::default()
        }
    }

    /// when failing to parse json from a reqwest response, returns the error in the `error` key
    pub fn parse_response_error(message: &str, error: reqwest::Error) -> Res {
        Res {
            status: 500,
            message: message.to_string(),
            error: error.to_string(),
            ..Default::default()
        }
    }

    /// When something has failed internally that shouldn't have failed
    pub fn internal_server_error(message: &str, error: Error) -> Res {
        Res {
            status: 500,
            message: message.to_string(),
            error: error.to_string(),
            ..Default::default()
        }
    }
}

impl IntoResponse for Res {
    /// Custom implementation for into_response so lambda_http can return a response in the correct format
    /// If it's 400 or above will log to stderr which will count as a failed execution in AWS lambda stats
    fn into_response(self) -> Response<Body> {
        let body = if !self.body.is_empty() {
            self.body
        } else {
            serde_json::to_string(&self).expect("failed to convert struct to json string")
        };

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

#[cfg(test)]
mod tests {
    use lambda_http::IntoResponse;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use super::Res;
    #[test]
    fn response_ok() {
        let res = Res::ok("test message response").into_response();

        let expected = json!({ "message": "test message response" });

        assert_eq!(res.status(), 200);
        assert_eq!(res.body(), expected.into_response().body());
    }

    #[test]
    fn ok_with_body() {
        let body = json!({"name": "test", "type": "ok_body"});
        let body_serialized = serde_json::to_string(&body).expect("failed to parse json body");

        let res = Res::ok_body(&body_serialized).into_response();

        assert_eq!(res.status(), 200);
        assert_eq!(res.body(), body.into_response().body());
    }

    #[test]
    fn parse_body_error() {
        #[derive(Serialize, Deserialize, Debug, Default)]
        struct Person {
            name: String,
        }

        let invalid_json = "name: bill";

        // Will only parse if it errors and runs the assertions below
        let person: Person = match serde_json::from_str(invalid_json) {
            Ok(v) => v,
            Err(error) => {
                let res = Res::parse_body_error(error);
                assert_eq!(res.status, 400);
                assert_eq!(res.message, "failed to parse json body");
                assert_eq!(res.error, "expected ident at line 1 column 2");
                Person {
                    name: "error worked".to_string(),
                }
            }
        };
        assert_eq!(person.name, "error worked".to_string());
    }

    #[test]
    fn bad_request() {
        let res = Res::bad_request("testing a bad request").into_response();
        assert_eq!(
            res.body(),
            json!({"message": "testing a bad request"})
                .into_response()
                .body()
        )
    }

    #[test]
    fn parse_response_error() {
        let error = reqwest::get("fakeurl")
            .err()
            .expect("failed to generate error");
        let res = Res::parse_response_error("testing a reqwest error", error);
        assert_eq!(res.message, "testing a reqwest error");
        assert_eq!(res.error, "relative URL without a base");

        let response = res.into_response();
        assert_eq!(response.status(), 500);
    }

    #[test]
    fn internal_server_error() {
        let error = std::io::Error::new(std::io::ErrorKind::Other, "test internal error");
        let res = Res::internal_server_error(
            "testing an arbitrary internal server error",
            Box::new(error),
        );
        assert_eq!(res.message, "testing an arbitrary internal server error");
        assert_eq!(res.error, "test internal error");

        let response = res.into_response();
        assert_eq!(response.status(), 500);
    }
}
