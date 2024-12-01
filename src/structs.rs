use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PictureOfTheDay {
    pub title: String,
    pub copyright: Option<String>,
    pub explanation: String,
    pub hdurl: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordMessage {
    pub username: String,
    pub embeds: Vec<DiscordMessageEmbed>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordMessageEmbed {
    pub title: String,
    pub description: String,
    pub fields: Vec<DiscordMessageEmbedField>,
    pub image: DiscordMessageEmbedImage,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordMessageEmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordMessageEmbedImage {
    pub url: String,
}

#[derive(Deserialize)]
pub struct Request {}

#[derive(Serialize)]
pub struct Response {
    pub req_id: String,
    pub msg: String,
}