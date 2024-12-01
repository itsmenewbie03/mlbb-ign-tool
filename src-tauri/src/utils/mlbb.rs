use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct CheckUidResp {
    code: String,
    msg: String,
    data: Data,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code, non_snake_case)]
struct Data {
    isValid: u8,
    nickName: String,
    isNewUser: u8,
}

pub async fn get_ign(game_id: &str, zone_id: &str) -> Option<String> {
    let url = "https://api.jollymax.com/jolly-gateway/topup/order/check-uid";
    let cli = reqwest::Client::new();
    let mut body = HashMap::new();
    body.insert("token", "503f6083a0564c07a58412f584297706");
    body.insert("jmsId", "");
    body.insert("appId", "APP20210608084718702");
    body.insert("systemFlag", "1.0");
    body.insert("roleName", "");
    body.insert("country", "ph");
    body.insert("language", "en");
    body.insert("appAlias", "mlbb_diamonds");
    body.insert("platformName", "");
    body.insert("serverId", zone_id);
    body.insert("goodsId", "G20230807113729602");
    body.insert("goodsSnapshotId", "");
    body.insert("payTypeId", "1029606");
    body.insert("userId", game_id);
    body.insert("activityId", "");
    body.insert("serverName", "");
    body.insert("domain", "www.jollymax.com");
    body.insert("deviceId", "2a8ceaef182240fa94cc86dcc73acd8e");
    let req = cli.post(url).json(&body);
    let resp = req.send().await;
    if resp.is_err() {
        return None;
    }
    // INFO: safe to unwrap
    let resp = resp.unwrap();
    let json_data = resp.json::<CheckUidResp>().await;
    match json_data {
        Ok(data) => {
            if data.data.isValid == 1 {
                Some(data.data.nickName)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
