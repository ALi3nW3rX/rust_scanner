use clap::Parser;
//use indicatif::ProgressBar;
use std::io::{self, BufRead};
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Ali3nW3rX Rust Scanner!!!
#[derive(Parser, Debug)]
#[command(name = "Ali3nW3rX")]
#[command(author = "Josh W. <UcanEatMyAss@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "A simple subdomain scanner written in Rust.")]
struct Args {
    /// Choose a wordlist to use
    #[arg(short, long, default_value = "wordlist.txt")]
    wordlist: String,

    #[arg(short, long)]
    domain: String,

    #[arg(short, long, default_value = "10")]
    threads: u16,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    let domain = args.domain.to_string();
    let wordlist = args.wordlist.to_string();

    let input = std::fs::File::open(wordlist).unwrap();
    let buffered = std::io::BufReader::new(input);

    let mut handles = Vec::new();
    let semaphore = Arc::new(Semaphore::new(args.threads as usize));

    for line in buffered.lines() {
        match line {
            Ok(subdomain) => {
                let domain = domain.clone();
                let permit = semaphore.clone();

                let handle = tokio::spawn(async move {
                    let _permit = permit.acquire().await.unwrap();
                    check_subdomain(subdomain, domain).await;
                });

                handles.push(handle);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}

async fn check_subdomain(subdomain: String, domain: String) {
    let url = format!("https://{}.{}", subdomain, domain);
    /*
    let _client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Linux; Android 13; SM-G998B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36")
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap();
    */
    let resp = reqwest::get(&url).await;

    match resp {
        Ok(response) => match response.status().as_u16() {
            200 => println!("{} is a valid subdomain", url),
            301 | 302 | 303 | 307 | 308 => {
                println!(
                    "{} is a valid subdomain, but redirects to {}",
                    url,
                    response
                        .headers()
                        .get("Location")
                        .unwrap()
                        .to_str()
                        .unwrap()
                )
            }
            400 => {}
            401 => println!("{} is a valid subdomain, but requires authentication", url),
            403 => println!("{} is a valid subdomain, but is forbidden", url),
            404 => {}
            500 => println!(
                "{} is a valid subdomain, but the server is having issues",
                url
            ),
            502 => println!("{} is a valid subdomain, but the server is down", url),
            503 => println!("{} is a valid subdomain, but the server is down", url),
            _ => {}
        },
        Err(_) => {}
    }
}


