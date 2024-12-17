pub mod structs {
    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub struct PictureOfTheDay {
        pub title: String,
        pub copyright: Option<String>,
        pub explanation: String,
        pub url: String,
        pub hdurl: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct DiscordMessage {
        pub username: String,
        pub embeds: Vec<DiscordMessageEmbed>,
    }

    impl DiscordMessage {
        pub fn new(model: PictureOfTheDay) -> Self {
            let fields = match model.copyright {
                Some(copyright) => vec![DiscordMessageEmbedField {
                    name: "Copyright".to_owned(),
                    value: copyright,
                    inline: true,
                }],
                None => vec![],
            };
            let image = model.hdurl.unwrap_or(model.url);

            DiscordMessage {
                username: "APOD".to_string(),
                embeds: vec![DiscordMessageEmbed {
                    title: model.title,
                    description: model.explanation,
                    image: DiscordMessageEmbedImage { url: image },
                    fields: fields,
                }],
            }
        }
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

    #[derive(Error, Debug)]
    pub enum CustomError {
        #[error("Reqwest error")]
        Reqwest(#[from] reqwest::Error),
        #[error("Serde JSON error")]
        SerdeJson(#[from] serde_json::Error),
    }
}
