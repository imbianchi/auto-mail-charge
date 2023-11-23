use serde_json::Value;
use std::process::Command;

pub fn get_usd_exchange_rate() -> Result<f64, Box<dyn std::error::Error>> {
    let output = Command::new("curl")
        .arg("https://open.er-api.com/v6/latest/USD")
        .output()?;

    if output.status.success() {
        let response: Value = serde_json::from_slice(&output.stdout)?;
        let usd_exchange_rate = response["rates"]["BRL"]
            .as_f64()
            .ok_or("BRL rate not found")?;

        Ok((usd_exchange_rate * 1000.0).round() / 1000.0)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.into())
    }
}
