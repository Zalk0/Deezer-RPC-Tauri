use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Artist {
    #[serde(rename = "ART_NAME")]
    pub name: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct CurrentSong {
    #[serde(rename = "SNG_TITLE")]
    pub track: String,
    #[serde(rename = "SNG_ID")]
    pub track_id: String,
    #[serde(rename = "ARTISTS")]
    pub artists: Option<Vec<Artist>>,
    #[serde(rename = "ART_NAME")]
    pub primary_artist: String,
    #[serde(rename = "ART_ID")]
    pub artist_id: String,
    #[serde(rename = "ALB_TITLE")]
    pub album: String,
    #[serde(rename = "ALB_ID")]
    pub album_id: String,
    #[serde(rename = "IS_PLAYING")]
    pub is_playing: bool,
    #[serde(rename = "SNG_TIME")]
    pub track_time: i64,
    #[serde(rename = "REM_TIME")]
    pub remaining_time: i64,
}
