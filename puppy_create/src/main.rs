#[macro_use]
extern crate lazy_static;

use core::{bad_request, LambdaError, PuppyDto, PuppyService};

use lambda_http::{handler, lambda, Context, IntoResponse, Request, RequestExt};

lazy_static! {
    static ref SERVICE: PuppyService = PuppyService::new();
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    lambda::run(handler(move |req, con| {
        my_handler(req, con, SERVICE.clone())
    }))
    .await?;

    Ok(())
}

async fn my_handler(
    e: Request,
    _: Context,
    service: PuppyService,
) -> Result<impl IntoResponse, LambdaError> {
    // Validate request
    // Do logic
    // Save to DB
    // LOG
    // Send to queue
    // Respond to user
    match e.payload::<PuppyDto>() {
        Ok(val) => match val {
            Some(x) => {
                service.create(&x).await?;
                service.bark::<PuppyDto>(&x).await?;
                Ok(x.into_response())
            }
            None => Ok(bad_request("No puppy was supplied".to_string())),
        },
        Err(err) => Ok(bad_request(format!(
            "Err: {}, Request{}",
            err,
            format!("{:?}", e)
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::{http::StatusCode, Body};

    // #[tokio::test]
    // async fn no_body() {
    //     let result = my_handler(Request::default(), Context::default())
    //         .await
    //         .expect("Expected Ok(_) value")
    //         .into_response();
    //     assert_eq!(
    //         result.body(),
    //         &Body::Text("{\"message\":\"No input!\"}".to_string())
    //     );
    //     assert_eq!(result.status(), StatusCode::OK);
    // }
}
