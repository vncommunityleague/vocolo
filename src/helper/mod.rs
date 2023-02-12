pub mod osu;

pub async fn init() {
    osu::OsuHelper::init().await;
}
