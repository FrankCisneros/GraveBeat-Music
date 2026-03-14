use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct DiscordClient {
    client: DiscordIpcClient,
}

impl DiscordClient {
    pub fn new(client_id: &str) -> Self {
        let mut client = DiscordIpcClient::new(client_id);
        client.connect().ok();

        Self { client }
    }

    pub fn set_activity(
        &mut self,
        title: &str,
        artist: &str,
        duration: i64,
        current_time: i64,
        playing: bool,
    ) {
        let mut activity = activity::Activity::new()
            .details(title)
            .state(artist)
            .assets(
                activity::Assets::new()
                    .large_image("app_icon")
                    .large_text("JMusic"),
            );

        if playing {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let start = now - current_time;
            let end = start + duration;

            activity = activity.timestamps(
                activity::Timestamps::new()
                    .start(start)
                    .end(end),
            );
        }

        let _ = self.client.set_activity(activity);
    }
}