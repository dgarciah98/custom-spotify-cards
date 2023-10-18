use base64::{engine::general_purpose, Engine};
use common::model::{AccessToken, Artist, Track};
use gloo_net::{
    http::{Request, Response},
    Error,
};

pub(crate) async fn get_song(id: String, token: AccessToken) -> Result<Track, Error> {
    let bearer = format!("{} {}", token.token_type, token.access_token);
    let res = Request::get(&format!("https://api.spotify.com/v1/tracks/{id}"))
        .header("Authorization", &bearer)
        .send()
        .await?;
    if res.status() != 200 {
        let value: serde_json::Value = serde_json::from_str(&res.text().await?)?;
        Err(Error::GlooError(value.get("error").unwrap().get("message").unwrap().to_string()))
    } else {
        res.json::<Track>().await
    }
}

pub(crate) async fn get_artist(id: String, token: AccessToken) -> Result<Artist, Error> {
    let bearer = format!("{} {}", token.token_type, token.access_token);
    Request::get(&format!("https://api.spotify.com/v1/artists/{id}"))
        .header("Authorization", &bearer)
        .send()
        .await?
        .json::<Artist>()
        .await
}

pub(crate) async fn get(url: String) -> Result<Response, Error> {
    Request::get(&url).send().await
}

pub(crate) async fn authorize() -> Result<AccessToken, Error> {
    let client_id = std::option_env!("CLIENT_ID")
        .expect("No token found. Please provide your client ID from Spotify Developer API Portal");
    let secret = std::option_env!("CLIENT_SECRET").expect(
        "No token found. Please provide your client secret from Spotify Developer API Portal",
    );
    let b64 = general_purpose::STANDARD.encode(format!("{}:{}", client_id, secret));
    let basic = "Basic ".to_owned() + &b64;

    Request::post("https://accounts.spotify.com/api/token")
        .header("Authorization", &basic)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .query([("grant_type", "client_credentials")])
        .send()
        .await?
        //.expect("Authorization API call failed")
        .json::<AccessToken>()
        .await
    //.expect("Could not deserialize response")
}
