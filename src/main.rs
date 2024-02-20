use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ruzz")]
struct Opt {
    #[structopt(short, long, default_value = "http://localhost")]
    url: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    println!("Target URL: {}", opt.url);

}
