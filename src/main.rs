mod structs;

use dotenv::dotenv;
use lambda_runtime::{service_fn, tracing, Error, LambdaEvent};
use structs::{
    DiscordMessage, DiscordMessageEmbed, DiscordMessageEmbedField, DiscordMessageEmbedImage,
    PictureOfTheDay, Request, Response,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serde JSON error")]
    SerdeJson(#[from] serde_json::Error),
}

#[tokio::main]
async fn main() -> Result<(), CustomError> {
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

    let picture_of_the_day = fetch_astronomy_picture_of_the_day().await?;

    let _ = send_picture_of_the_day_to_discord(picture_of_the_day).await?;

    Ok(())
}

async fn fetch_astronomy_picture_of_the_day() -> Result<PictureOfTheDay, CustomError> {
    let res =
        reqwest::get(std::env::var("NASA_APOD_API").expect("Unexpected NASA_APOD_API")).await?;

    let body: String = res.text().await?;

    let picture_of_the_day: PictureOfTheDay = serde_json::from_str(&body)?;

    Ok(picture_of_the_day)
}

async fn send_picture_of_the_day_to_discord(
    picture_of_the_day: PictureOfTheDay,
) -> Result<(), CustomError> {
    let fields = match picture_of_the_day.copyright {
        Some(copyright) => vec![DiscordMessageEmbedField {
            name: "Copyright".to_owned(),
            value: copyright,
            inline: true,
        }],
        None => vec![],
    };

    let message = DiscordMessage {
        username: "APOD".to_string(),
        embeds: vec![DiscordMessageEmbed {
            title: picture_of_the_day.title,
            description: picture_of_the_day.explanation,
            image: DiscordMessageEmbedImage {
                url: picture_of_the_day.hdurl,
            },
            fields: fields,
        }],
    };
    let client = reqwest::Client::new();
    let _ = client
        .post(std::env::var("DISCORD_WEB_HOOK").expect("Unexpected DISCORD_WEB_HOOK"))
        .json(&message)
        .send()
        .await?;

    Ok(())
}
