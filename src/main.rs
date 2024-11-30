use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serde JSON error")]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize)]
struct PictureOfTheDay {
    title: String,
    copyright: String,
    explanation: String,
    hdurl: String,
}

#[derive(Serialize, Deserialize)]
struct DiscordMessage {
    username: String,
    embeds: Vec<DiscordMessageEmbed>,
}

#[derive(Serialize, Deserialize)]
struct DiscordMessageEmbed {
    title: String,
    description: String,
    fields: Vec<DiscordMessageEmbedField>,
    image: DiscordMessageEmbedImage,
}

#[derive(Serialize, Deserialize)]
struct DiscordMessageEmbedField {
    name: String,
    value: String,
    inline: bool,
}

#[derive(Serialize, Deserialize)]
struct DiscordMessageEmbedImage {
    url: String,
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
    let message = DiscordMessage {
        username: "APOD".to_string(),
        embeds: vec![DiscordMessageEmbed {
            title: picture_of_the_day.title,
            description: picture_of_the_day.explanation,
            image: DiscordMessageEmbedImage {
                url: picture_of_the_day.hdurl,
            },
            fields: vec![DiscordMessageEmbedField {
                name: "Copyright".to_owned(),
                value: picture_of_the_day.copyright,
                inline: true,
            }],
        }],
    };

    let client = reqwest::Client::new();
    let _ = client
        .post(std::env::var("DISCORD_WEB_HOOK").expect("Unexpected DISCORD_WEB_HOOK"))
        .json(&message)
        .send()
        .await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    dotenv().ok();

    let picture_of_the_day = fetch_astronomy_picture_of_the_day().await?;

    let _ = send_picture_of_the_day_to_discord(picture_of_the_day).await?;

    Ok(())
}
