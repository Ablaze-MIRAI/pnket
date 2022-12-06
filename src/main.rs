use structopt::StructOpt;

mod download;
mod text;


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
        if let Err(err) = text::text(url) {
            println!("{:?}", err);
        }
    } else {
        if let Err(err) = download::download(url, output) {
            println!("{:?}", err);
        }
    }
}
