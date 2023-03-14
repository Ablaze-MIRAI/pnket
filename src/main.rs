use structopt::StructOpt;

mod download;
mod text;

#[derive(StructOpt)]
struct Opt {
    url: Vec<String>,
    #[structopt(short = "o", long, default_value = "")]
    output: String,
    #[structopt(short = "c", long)]
    text: bool,
    #[structopt(short = "z", long)]
    tzst: bool,
}

fn main() {
    let opt = Opt::from_args();
    let url: Vec<&str> = opt.url.iter().map(|u| u.as_str()).collect();
    let mut output: &str = &opt.output;

    for i in 0..url.len() {
        if opt.text == true {
            if let Err(err) = text::text(url[i]) {
                println!("{:?}", err);
            }
        } else {
            if let Err(err) = download::download(url[i], output, opt.tzst) {
                println!("{:?}", err);
            }
        }
        output = "";
    }
}
