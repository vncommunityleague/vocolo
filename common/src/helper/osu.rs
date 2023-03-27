use rosu_v2::Osu;

use crate::constants::EnvironmentVariable;

pub struct OsuHelper {
    pub osu: Osu,
}

impl OsuHelper {
    pub async fn init() -> Self {
        let osu_client_id = EnvironmentVariable::OSU_CLIENT_ID.value_with_type::<u64>();
        let osu_client_secret = EnvironmentVariable::OSU_CLIENT_SECRET.value();

        Self {
            osu: match Osu::new(osu_client_id, osu_client_secret).await {
                Ok(client) => client,
                Err(err) => panic!("Error while creating osua! client: {}", err),
            },
        }
    }
}
