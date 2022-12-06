use anyhow::Result;
use std::fs::File;
use std::io;

#[tokio::main]
pub async fn download(url: &str, output: &str) -> Result<()> {
    let filename = if output.is_empty() {
        if url.ends_with("/") {
            &url[..url.rfind('/').unwrap()].split("/").last().unwrap()
        } else {
            url.split("/").last().unwrap()
        }
    } else {
        output
    };
    
    let client = reqwest::Client::new();
    let get = client.get(url)
        .send()
        .await?;
    let bytes = get
        .bytes()
        .await?;
    
    let mut out = File::create(filename)?;
    io::copy(&mut bytes.as_ref(), &mut out)?;

    Ok(())
}
