use clap::Parser;
use reqwest::Client;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio::time::{self, Duration};

#[derive(Parser)]
struct Cli {
    url: String,
    wordlist: String,
}
fn get_urls(url: String, path: String) -> Vec<String> {
    let mut wordlist = Vec::new();
    let file = File::open(path).expect("File not found!");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        wordlist.push(format!("{}/{}", url, line.unwrap()));
    }
    wordlist
}

async fn check_urls(urls: Vec<String>) {
    let client = Client::new();

    for url in urls {
        let url_clone = url.to_owned();
        let request = client.head(url).send();
        let response = time::timeout(Duration::from_secs(5), request).await;

        match response {
            Ok(Ok(res)) => {
                if res.status().is_success() {
                    println!("Page found: {} {:?}", url_clone, res.status());
                }
            }
            Ok(Err(_e)) => {
                println!("Error occurred {}", url_clone);
            }
            Err(_) => {
                println!("Timeout occurred {}", url_clone);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let urls = get_urls(args.url, args.wordlist);
    check_urls(urls).await;
}
