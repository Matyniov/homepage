use reqwest::get;
const IP_URL: &str = "https://api.ipify.org/";

pub async fn get_ip() -> Option<(u8, u8, u8, u8)> {
    let parts: Vec<u8> = get(IP_URL)
        .await
        .ok()?
        .text()
        .await
        .ok()
        .and_then(|inner| inner.split(".").map(|el| str::parse(el).ok()).collect())?;

    if parts.len() == 4 {
        Some((parts[0], parts[1], parts[2], parts[3]))
    } else {
        None
    }
}

pub fn ip_to_string(ip: (u8, u8, u8, u8)) -> String {
    format!("{}.{}.{}.{}", ip.0, ip.1, ip.2, ip.3)
}
