//! `boing metrics register` — Register contract for dApp incentive tracking via RPC.

use serde_json::json;

pub async fn run(rpc_url: &str, contract: &str, owner: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "boing_registerDappMetrics",
        "params": [contract, owner]
    });

    let resp = client
        .post(rpc_url)
        .json(&body)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Cannot connect to {}: {}. Run `boing dev` first.", rpc_url, e))?;

    if !resp.status().is_success() {
        anyhow::bail!("RPC returned {}: {}", resp.status(), resp.text().await?);
    }

    let text = resp.text().await?;
    let parsed: serde_json::Value = serde_json::from_str(&text)?;

    if let Some(err) = parsed.get("error") {
        let msg = err.get("message").and_then(|v| v.as_str()).unwrap_or("Unknown error");
        anyhow::bail!("Registration failed: {}", msg);
    }

    let result = parsed.get("result").and_then(|r| r.get("registered"));
    if result.and_then(|v| v.as_bool()).unwrap_or(false) {
        println!("✓ Contract registered for dApp incentive tracking");
        println!("  contract: {}", contract);
        println!("  owner: {}", owner);
    } else {
        println!("Registration may have succeeded; check node logs.");
    }

    Ok(())
}
