use structopt::StructOpt;

mod download;
mod text;


#[derive(StructOpt)]
struct Opt {
    url: String,
    #[structopt(short = "o", long, default_value = "")]
    output: String,
    #[structopt(short = "p", long, default_value = "1")]
    procs: usize,
    #[structopt(short = "c", long)]
    text: bool,
    #[structopt(short = "z", long)]
    tzst: bool,
}

fn main() {
    let opt = Opt::from_args();
    let url: &str = &opt.url;
    let output: &str = &opt.output;
    let procs: usize = if opt.procs < 1 {
            1
        } else {
            opt.procs
        };
    
    if opt.text == true {
        if let Err(err) = text::text(url) {
            println!("{:?}", err);
        }
    } else {
        if let Err(err) = download::download(url, output, opt.tzst, procs) {
            println!("{:?}", err);
        }
    }
}
