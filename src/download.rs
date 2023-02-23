use anyhow::Result;
use std::fs::File;
use std::io;
use indicatif;
use futures_util::StreamExt;

mod tzst;

#[tokio::main]
pub async fn download(url: &str, output: &str, tzst_bool: bool) -> Result<()> {
    let filename: &str = if output.is_empty() {
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

    let length = get.content_length().unwrap_or(0);
    let pb = indicatif::ProgressBar::new(length);
    pb.set_style(
        indicatif::ProgressStyle::with_template(&format!("{}\n[{{elapsed_precise:.cyan/blue}}] [{{wide_bar:.green}}] {{bytes}}/{{total_bytes}}\n\n", filename))
            .unwrap()
            .progress_chars("=> "));
    pb.set_position(0);

    let mut out = File::create(filename)?;
    let mut stream = get.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let bytes = chunk?;
        pb.inc(bytes.len() as u64);
        io::copy(&mut bytes.as_ref(), &mut out)?;
    }

    pb.finish();

    if tzst_bool {
        if rmext(filename) == filename {
            if let Err(err) = tzst::tzst(filename, &format!("{}{}", filename, ".d")) {
                println!("{:?}", err);
            }
        } else {
            if let Err(err) = tzst::tzst(filename, rmext(filename)) {
                println!("{:?}", err);
            }
        }
    }

    Ok(())
}

fn rmext(filename: &str) -> &str {
    let mut filename = filename;
    while let Some(i) = filename.rfind('.') {
        if filename.ends_with(".pkg") {
            break;
        } else {
            filename = &filename[..i];
        }
    }
    return filename;
}
