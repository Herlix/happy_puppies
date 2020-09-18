use dynomite::{
    dynamodb::PutItemInput,
    dynamodb::{DynamoDb, DynamoDbClient},
    retry::Policy,
    retry::RetryingDynamoDb,
    Item, Retries,
};
use lambda_http::{http::StatusCode, Body, IntoResponse, Response};
use rusoto_core::Region;
use rusoto_sqs::{GetQueueUrlRequest, SendMessageRequest, Sqs, SqsClient};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type LambdaError = Box<dyn std::error::Error + Send + Sync + 'static>;
#[derive(Clone)]
pub struct PuppyService {
    sqs: SqsClient,
    db: RetryingDynamoDb<DynamoDbClient>,
    table_name: String,
}

impl PuppyService {
    pub fn new() -> Self {
        let db = DynamoDbClient::new(Default::default()).with_retries(Policy::default());
        let sqs = SqsClient::new(Region::default());
        let table_name =
            std::env::var("DYNAMODB_TABLE").expect("DYNAMODB_TABLE should be set in env!");
        PuppyService {
            db,
            sqs,
            table_name,
        }
    }

    pub async fn create(&self, puppy: &PuppyDto) -> Result<(), LambdaError> {
        let pup = Puppy {
            id: Uuid::new_v4(),
            name: puppy.name.clone(),
            breed: puppy.breed.clone(),
            age: puppy.age as i16,
        };

        self.db
            .put_item(PutItemInput {
                item: pup.clone().into(),
                table_name: self.table_name.clone(),
                ..PutItemInput::default()
            })
            .await?;
        Ok(())
    }

    pub async fn bark<T>(&self, msg: &T) -> Result<(), LambdaError>
    where
        T: Serialize,
    {
        let queue_url = self
            .sqs
            .get_queue_url(GetQueueUrlRequest {
                queue_name: std::env::var("SQS_NAME").unwrap(),
                queue_owner_aws_account_id: None,
            })
            .await?
            .queue_url
            .unwrap();
        self.sqs
            .send_message(SendMessageRequest {
                message_body: format!("\"body\":\"{}\"", serde_json::to_string(&msg)?),
                queue_url,
                ..SendMessageRequest::default()
            })
            .await?;
        Ok(())
    }
}

#[derive(Clone, Item)]
struct Puppy {
    #[dynomite(partition_key)]
    id: Uuid,
    #[dynomite(sort_key)]
    name: String,
    breed: String,
    age: i16,
}

#[derive(Serialize, Deserialize)]
pub struct PuppyDto {
    pub name: String,
    pub breed: String,
    pub age: i8,
}

impl IntoResponse for PuppyDto {
    fn into_response(self) -> Response<Body> {
        Response::new(Body::Text(serde_json::to_string(&self).unwrap()))
    }
}

pub fn bad_request(msg: String) -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST.as_u16())
        .body(Body::Text(msg))
        .unwrap()
}
