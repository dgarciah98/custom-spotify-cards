use common::model::{CardData, AccessToken};
use gloo::utils::errors::JsError;
use gloo_net::Error;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::HtmlDocument;
use yew::UseStateHandle;

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

pub(crate) async fn fetch_data(id: String, token: UseStateHandle<Option<AccessToken>>) -> Result<CardData, Error> {
    let mut mut_token: AccessToken;
	if (*token).is_none() { mut_token = api::authorize().await?; }
	else { mut_token = (*token).clone().unwrap(); }
	
    let track: common::model::Track = match api::get_song(id.clone(), mut_token.clone()).await {
		Ok(res) => { res }
		Err(err) => {
			match err.to_string().as_str() {
				"The access token expired" => {
					let new_token = api::authorize().await?;
					mut_token = new_token.clone();
					api::get_song(id.clone(), new_token).await?				
				}
				_ => { panic!() }				
			}
		}
	};
	let common::model::Track { id, name, album, .. } = track.clone();
    let artist_id = track.artists.first().unwrap().id.to_owned();
    let artist = api::get_artist(artist_id, mut_token.clone()).await?;
	let image_data = album.images.first().unwrap();
    let image_bytes = api::get(image_data.url.to_owned()).await?.binary().await?;
	token.set(Some(mut_token));
    Ok(common::model::CardData {
		track_id: id,
        name,
        album: album.name,
        album_type: album.album_type,
        artists: track.artists(),
        genres: artist.genres().unwrap(),
        jacket_size: image_data.width,
		jacket_bytes: image_bytes
    })
}
