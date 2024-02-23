use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ruzz")]
struct Opt {
    #[structopt(short, long, default_value = "http://localhost")]
    url: String,
}

async fn fuzz_url(url: &str) {
    let client = reqwest::Client::new();
    match client.get(url).send().await {
        Ok(response) => {
            println!("Received response for {}: {}", url, response.status());
            // Further processing based on response
        },
        Err(e) => println!("Error requesting {}: {}", url, e),
    }
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    println!("Target URL: {}", opt.url);

    fuzz_url(&opt.url).await;
}

