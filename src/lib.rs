use anyhow::Result;

mod download;
mod text;

pub async fn download(url: &str, output: &str, tzst_bool: bool) -> Result<()> {
    download::download(url, output, tzst_bool)?;

    Ok(())
}

pub async fn text(url: &str) -> Result<()> {
    text::text(url)?;

    Ok(())
}
