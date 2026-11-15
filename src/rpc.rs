use discord_rich_presence::activity::{
    Activity, ActivityType, Assets, StatusDisplayType, Timestamps,
};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Client {
    client: DiscordIpcClient,
}

impl Client {
    pub fn new() -> Self {
        let mut client = DiscordIpcClient::new("1173321053766963310");
        client.connect().unwrap_or_default();
        Client { client }
    }

    pub fn clear_activity(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client.clear_activity()?;
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_activity()?;
        self.client.close()?;
        Ok(())
    }

    pub fn set_activity(
        &mut self,
        info: crate::types::CurrentSong,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let assets = Assets::new()
            // Album
            .large_image(format!(
                "https://api.deezer.com/album/{}/image?size=big",
                info.album_id
            ))
            .large_text(info.album)
            .large_url(format!("https://www.deezer.com/album/{}", info.album_id));
        let end_time =
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64 + info.remaining_time;
        let payload = Activity::new()
            .activity_type(ActivityType::Listening)
            // Choose what is displayed in members list on Discord (default to artist like Spotify)
            .status_display_type(StatusDisplayType::State)
            // Artist
            .state(match info.artists {
                Some(artists) => artists
                    .iter()
                    .map(|a| a.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                None => info.primary_artist,
            })
            .state_url(format!("https://www.deezer.com/artist/{}", info.artist_id))
            // Song
            .details(info.track)
            .details_url(format!("https://www.deezer.com/track/{}", info.track_id))
            // Album
            .assets(assets)
            // Start + End timestamps
            .timestamps(
                Timestamps::new()
                    .start(end_time - info.track_time)
                    .end(end_time),
            );
        self.client.set_activity(payload)?;
        Ok(())
    }

    pub fn update_activity(
        &mut self,
        info: crate::types::CurrentSong,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if info.is_playing {
            self.set_activity(info)?;
        } else {
            self.clear_activity()?;
        }
        Ok(())
    }
}
