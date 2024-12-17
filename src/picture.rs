pub mod picture {

    use crate::entities::structs::{CustomError, PictureOfTheDay};

    pub async fn find_astronomy_picture_of_the_day<F>(
        request: F,
    ) -> Result<PictureOfTheDay, CustomError>
    where
        F: std::future::Future<Output = Result<String, CustomError>>,
    {
        let body = request.await?;

        let picture_of_the_day: PictureOfTheDay = serde_json::from_str(&body)?;

        Ok(picture_of_the_day)
    }

    pub async fn request_astronomy_picture_of_the_day(
        nasa_apod_api_url: String,
    ) -> Result<String, CustomError> {
        let res =
            reqwest::get(nasa_apod_api_url).await?;
        let text = res.text().await?;
        
        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::structs::{CustomError, PictureOfTheDay};
    use crate::gen::gen;

    #[tokio::test]
    async fn find_astronomy_picture_of_the_day_successfully() -> Result<(), CustomError> {
        let mut rng = rand::thread_rng();
        let explanation = gen::string(&mut rng);
        let url = gen::string(&mut rng);
        let hdurl = gen::string(&mut rng);
        let title = gen::string(&mut rng);
        let copyright = gen::string(&mut rng);

        let json = format!(
            "{{\"explanation\":\"{}\",\"title\":\"{}\",\"url\":\"{}\", \"copyright\": \"{}\", \"hdurl\": \"{}\"}}",
            explanation, title, url, copyright, hdurl
        );

        let response = picture::find_astronomy_picture_of_the_day(async { Ok(json) }).await?;

        let picture_of_the_day = PictureOfTheDay {
            title,
            copyright: Some(copyright),
            explanation,
            hdurl: Some(hdurl),
            url,
        };

        assert_eq!(response, picture_of_the_day);

        Ok(())
    }
}
