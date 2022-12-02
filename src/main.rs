use structopt::StructOpt;
use anyhow::Result;
use std::fs::File;
use std::io;


#[derive(StructOpt)]
struct Opt {
    url: String,
    #[structopt(short = "o", long, default_value = "")]
    output: String,
    #[structopt(short = "c", long)]
    text: bool,
}

fn main() {
    let opt = Opt::from_args();
    let url: &str = &opt.url;
    let output: &str = &opt.output;
    
    if opt.text == true {
        if let Err(err) = text(url) {
            println!("{:?}", err);
        }
    } else {
        if let Err(err) = download(url, output) {
            println!("{:?}", err);
        }
    }
}

#[tokio::main]
async fn download(url: &str, output: &str) -> Result<()> {
    let filename = if output.is_empty() {url.split("/").last().unwrap()} else {output};
    let client = reqwest::Client::new();
    let bytes = client.get(url)
        .send()
        .await?
        .bytes()
        .await?;
    let mut out = File::create(filename)?;
    io::copy(&mut bytes.as_ref(), &mut out)?;

    Ok(())
}

#[tokio::main]
async fn text(url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let content = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", content);

    Ok(())
}