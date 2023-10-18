use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    pub url: String,
    pub height: u16,
    pub width: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Album {
    pub album_type: String,
    pub images: Vec<Image>,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub genres: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub album: Album,
    pub artists: Vec<Artist>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CardData {
    pub track_id: String,
    pub name: String,
    pub album: String,
    pub album_type: String,
    pub artists: String,
    pub genres: String,
    pub jacket_size: u16,
    pub jacket_bytes: Vec<u8>,
}

impl Track {
    pub fn artists(&self) -> String {
        self.artists.clone().into_iter().map(|a| a.name).collect::<Vec<String>>().join(", ")
    }
}

impl Artist {
    pub fn genres(&self) -> Option<String> {
        if self.genres.is_some() {
            Some(
                self.genres
                    .to_owned()
                    .unwrap()
                    .into_iter()
                    .map(|s| format!("#{s}"))
                    .collect::<Vec<String>>()
                    .join(" "),
            )
        } else {
            None
        }
    }
}
