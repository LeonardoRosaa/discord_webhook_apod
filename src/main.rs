mod entities;
mod gen;
mod message;
mod picture;

use crate::entities::structs::{CustomError, DiscordMessage, Request, Response};
use crate::message::message::send_message_to_discord;
use crate::picture::picture::{
    find_astronomy_picture_of_the_day, request_astronomy_picture_of_the_day,
};
use dotenv::dotenv;
use lambda_runtime::{service_fn, tracing, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    fetch_picture_and_send_message().await?;
    tracing::init_default_subscriber();

    let func = service_fn(handler);
    let _ = lambda_runtime::run(func).await;

    Ok(())
}

pub(crate) async fn handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let result = fetch_picture_and_send_message().await;

    match result {
        Ok(_) => {
            let resp = Response {
                req_id: event.context.request_id,
                msg: format!("Sent message"),
            };

            Ok(resp)
        }
        Err(error) => panic!("{:?}", error),
    }
}

async fn fetch_picture_and_send_message() -> Result<(), CustomError> {
    dotenv().ok();

    let nasa_apod_api_url = std::env::var("NASA_APOD_API").expect("Unexpected NASA_APOD_API");
    let discord_web_hook = std::env::var("DISCORD_WEB_HOOK").expect("Unexpected DISCORD_WEB_HOOK");
    let picture_of_the_day =
        find_astronomy_picture_of_the_day(request_astronomy_picture_of_the_day(nasa_apod_api_url))
            .await?;

    let _ =
        send_message_to_discord(DiscordMessage::new(picture_of_the_day), discord_web_hook).await?;

    Ok(())
}
