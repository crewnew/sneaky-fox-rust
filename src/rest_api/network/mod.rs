use super::{Id, ENDPOINT, HTTP_CLIENT};
use crate::capture::pc_common::NetworkInfo;

lazy_static! {
    static ref NETWORKS_ENDPOINT: String = format!("{}/items/networks", ENDPOINT.as_str());
}

pub fn get_network_info(ssid: impl Into<String>) -> anyhow::Result<NetworkInfo> {
    #[derive(Debug, Deserialize)]
    struct Data {
        data: Vec<Id>,
    }
    #[derive(Serialize)]
    struct Network<'a> {
        name: &'a str,
    }

    let ssid = ssid.into();

    debug!("{}", ssid);

    let url = format!("{}?search={}", NETWORKS_ENDPOINT.as_str(), ssid);

    let mut networks = HTTP_CLIENT.get(url).send()?.json::<Data>()?.data;

    debug!("{:?}", networks);

    if networks.is_empty() {
        let id = HTTP_CLIENT
            .post(NETWORKS_ENDPOINT.as_str())
            .json(&Network { name: &ssid })
            .send()?
            .text()?;

        return Ok(NetworkInfo { id, name: ssid });
    }

    let id = networks.pop().unwrap().id;

    debug!("{}", id);

    Ok(NetworkInfo { id, name: ssid })
}
