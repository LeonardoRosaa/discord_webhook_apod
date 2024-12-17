pub mod message {
    use crate::entities::structs::{CustomError, DiscordMessage};

    pub async fn send_message_to_discord(
        message: DiscordMessage,
        web_hook_url: String,
    ) -> Result<(), CustomError> {
        let client = reqwest::Client::new();
        let _ = client.post(web_hook_url).json(&message).send().await?;
    
        Ok(())
    }
}