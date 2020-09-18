use core::LambdaError;

use serde::{Deserialize, Serialize};

use lambda_http::{handler, lambda, Body, Context, IntoResponse, Request, RequestExt, Response};

#[derive(Deserialize)]
pub struct CustomEvent {
    #[serde(rename = "firstName")]
    pub first_name: String,
}

#[derive(Serialize)]
pub struct CustomOutput {
    pub message: String,
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    lambda::run(handler(my_handler)).await?;
    Ok(())
}

// curl -i -d "{\"firstName\":\"bob\"}" -X POST https://jcwe0t2nzl.execute-api.eu-north-1.amazonaws.com/dev/ -H "content-type: application/json"
async fn my_handler(e: Request, _: Context) -> Result<impl IntoResponse, LambdaError> {
    // validate input
    // log
    // update puppy if exists
    // queue
    // respond
    match e.payload::<CustomEvent>() {
        Ok(val) => match val {
            Some(x) => Ok(CustomOutput {
                message: x.first_name,
            }),
            None => Ok(CustomOutput {
                message: "No input!".to_string(),
            }),
        },
        Err(err) => Ok(CustomOutput {
            message: format!("Err: {}, Request{}", err, format!("{:?}", e)),
        }),
    }
}

impl IntoResponse for CustomOutput {
    fn into_response(self) -> Response<Body> {
        Response::new(Body::Text(
            serde_json::to_string(&self).expect("Could not create a response from a CustomOutput"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use lambda_http::http::StatusCode;

    use super::*;

    #[tokio::test]
    async fn no_body() {
        let result = my_handler(Request::default(), Context::default())
            .await
            .expect("Expected Ok(_) value")
            .into_response();
        assert_eq!(
            result.body(),
            &Body::Text("{\"message\":\"No input!\"}".to_string())
        );
        assert_eq!(result.status(), StatusCode::OK);
    }
}
