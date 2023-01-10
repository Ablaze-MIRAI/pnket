use structopt::StructOpt;

mod download;
mod text;


#[derive(StructOpt)]
struct Opt {
    url: Vec<String>,
    #[structopt(short = "o", long, default_value = "")]
    output: Vec<String>,
    #[structopt(short = "c", long)]
    text: bool,
    #[structopt(short = "z", long)]
    tzst: bool,
}

fn main() {
    let opt = Opt::from_args();
    let url: Vec<&str> = opt.url.iter().map(|u| u.as_str()).collect();
    let mut output: Vec<&str> = opt.output.iter().map(|o| o.as_str()).collect();
    while output.len() < url.len() {
        output.push("");
    }

    for i in 0..url.len() {
        if opt.text == true {
            if let Err(err) = text::text(url[i]) {
                println!("{:?}", err);
            }
        } else {
            if let Err(err) = download::download(url[i], output[i], opt.tzst) {
                println!("{:?}", err);
            }
        }
    }
}
