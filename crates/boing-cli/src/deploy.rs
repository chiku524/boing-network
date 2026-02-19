//! `boing deploy` — Deploy to network via RPC.

use serde_json::json;
use tracing::info;

pub async fn run(rpc_url: &str, _path: &str) -> anyhow::Result<()> {
    // Check chain height to verify RPC is reachable
    let client = reqwest::Client::new();
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "boing_chainHeight",
        "params": []
    });

    let resp = client
        .post(rpc_url)
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            anyhow::anyhow!(
                "Cannot connect to {}: {}. Run `boing dev` first.",
                rpc_url,
                e
            )
        })?;

    if !resp.status().is_success() {
        anyhow::bail!("RPC returned {}: {}", resp.status(), resp.text().await?);
    }

    let text = resp.text().await?;
    let parsed: serde_json::Value = serde_json::from_str(&text)?;
    let height = parsed.get("result").and_then(|v| v.as_u64()).unwrap_or(0);

    info!("Connected to {} — chain height {}", rpc_url, height);
    println!("✓ Connected to {} — chain height {}", rpc_url, height);
    println!("  Deployment flow: submit ContractDeploy tx via boing_submitTransaction");
    println!("  Use boing_simulateTransaction to dry-run first");

    Ok(())
}
