use anyhow::Result;

#[tokio::main]
pub async fn text(url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let get = client.get(url)
        .send()
        .await?;
    let content = get
        .text()
        .await?;

    println!("{}", content);

    Ok(())
}
