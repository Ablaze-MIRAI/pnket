use anyhow::Result;
use futures_util::StreamExt;
use indicatif;

#[tokio::main]
pub async fn text(url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let get = client.get(url).send().await?;

    let length = get.content_length().unwrap_or(0);
    let pb = indicatif::ProgressBar::new(length);
    pb.set_style(
        indicatif::ProgressStyle::with_template(
            "[{elapsed_precise:.cyan/blue}] [{wide_bar:.green}] {bytes}/{total_bytes}",
        )
        .unwrap()
        .progress_chars("=> "),
    );
    pb.set_position(0);

    let mut stream = get.bytes_stream();

    let mut bytes: Vec<u8> = vec![];

    while let Some(chunk) = stream.next().await {
        let c_bytes = chunk?;
        pb.inc(c_bytes.len() as u64);
        bytes.extend(c_bytes);
    }

    pb.finish_and_clear();

    let content: &str = &String::from_utf8(bytes).unwrap();

    println!("{}", content);

    Ok(())
}
