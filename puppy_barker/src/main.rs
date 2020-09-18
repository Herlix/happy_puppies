#[macro_use]
extern crate lazy_static;

use core::{LambdaError, PuppyService};
use lambda::{handler_fn, Context};
use rusoto_sqs::MessageAttributeValue;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref SERVICE: PuppyService = PuppyService::new();
}

#[derive(Deserialize)]
struct MyMessage {
    /// <p>A map of the attributes requested in <code> <a>ReceiveMessage</a> </code> to their respective values. Supported attributes:</p> <ul> <li> <p> <code>ApproximateReceiveCount</code> </p> </li> <li> <p> <code>ApproximateFirstReceiveTimestamp</code> </p> </li> <li> <p> <code>MessageDeduplicationId</code> </p> </li> <li> <p> <code>MessageGroupId</code> </p> </li> <li> <p> <code>SenderId</code> </p> </li> <li> <p> <code>SentTimestamp</code> </p> </li> <li> <p> <code>SequenceNumber</code> </p> </li> </ul> <p> <code>ApproximateFirstReceiveTimestamp</code> and <code>SentTimestamp</code> are each returned as an integer representing the <a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a> in milliseconds.</p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The message's contents (not URL-encoded).</p>
    pub body: Option<String>,
    /// <p>An MD5 digest of the non-URL-encoded message body string.</p>
    pub md5_of_body: Option<String>,
    /// <p>An MD5 digest of the non-URL-encoded message attribute string. You can use this attribute to verify that Amazon SQS received the message correctly. Amazon SQS URL-decodes the message before creating the MD5 digest. For information about MD5, see <a href="https://www.ietf.org/rfc/rfc1321.txt">RFC1321</a>.</p>
    pub md5_of_message_attributes: Option<String>,
    /// <p>Each message attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-attributes.html">Amazon SQS Message Attributes</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    pub message_attributes: Option<::std::collections::HashMap<String, MessageAttributeValue>>,
    /// <p>A unique identifier for the message. A <code>MessageId</code>is considered unique across all AWS accounts for an extended period of time.</p>
    pub message_id: Option<String>,
    /// <p>An identifier associated with the act of receiving the message. A new receipt handle is returned every time you receive a message. When deleting a message, you provide the last received receipt handle to delete the message.</p>
    pub receipt_handle: Option<String>,
}

#[derive(Serialize)]
struct QueueResponse {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    lambda::run(handler_fn(move |req, con| {
        queue_handler(req, con, SERVICE.clone())
    }))
    .await?;

    Ok(())
}

#[derive(Serialize)]
struct Msg {
    message: String,
}
async fn queue_handler(
    item: MyMessage,
    _c: Context,
    service: PuppyService,
) -> Result<String, LambdaError> {
    // Liten to queue and log each message with a timestamp
    let msg = item.body.unwrap_or("inget?!".to_string());
    service
        .bark(&Msg {
            message: format!("Yeeey: {}", msg),
        })
        .await?;

    Ok(format!("Wow: {}", msg))
}
