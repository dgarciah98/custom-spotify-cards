use common::model::CardData;
use wasm_bindgen::JsValue;

use crate::api;

pub(crate) fn parse_uri(uri: String) -> Result<String, JsValue> {
    let get_id = |url: web_sys::Url| {
        url.pathname().split(|c| c == '/' || c == ':').last().unwrap().to_string()
    };
    let regex = regex::Regex::new(r"(^[a-zA-Z0-9]{22}$)").unwrap();
    if regex.is_match(&uri) {
        return Ok(uri);
    }

    let url = web_sys::Url::new(&uri);
    let url_res = match url {
        Ok(url) => Ok(get_id(url)),
        Err(err) => Err(err),
    };
    match url_res {
        Ok(id) => {
            if regex.is_match(&id) {
                Ok(id)
            } else {
                Err(JsValue::from_str("Invalid ID"))
            }
        }
        Err(err) => Err(err),
    }
}

pub(crate) async fn fetch_data(id: String) -> CardData {
    let token = api::authorize().await;
    let track = api::get_song(id, token.clone()).await;
	let common::model::Track { name, album, .. } = track.clone();
    let artist_id = track.artists.first().unwrap().id.to_owned();
    let artist = api::get_artist(artist_id, token).await;
	let image_data = album.images.first().unwrap();
    let image_bytes = api::get(image_data.url.to_owned()).await.binary().await.unwrap();
    common::model::CardData {
        name,
        album: album.name,
        album_type: album.album_type,
        artists: track.artists(),
        genres: artist.genres().unwrap(),
        jacket_size: image_data.width,
		jacket_bytes: image_bytes
    }
}
