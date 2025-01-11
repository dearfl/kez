use std::time::Duration;

use clap::Parser;
use kez::Client;

// By default this example tries to collect history matches with match_seq_num between 0-1000
// with a request interval of 10s
// An API KEY is required to use STEAM WEB API. You can accquire one from https://steamcommunity.com/dev/apikey
#[derive(Parser)]
pub struct Args {
    #[arg(long, default_value_t = 10000)]
    pub interval: u64,

    #[arg(long, default_value_t = 0)]
    pub start: u64,
    #[arg(long, default_value_t = 1000)]
    pub end: u64,

    pub key: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = Client::new(args.key)?;
    let mut start = args.start;
    let end = args.end;
    while start < end {
        println!("collecting matches starting from {}", start);
        let matches = client.history(start, 100).await?;

        // update start match_seq_num
        start = matches.iter().fold(start + 1, |init, mat| {
            std::cmp::max(init, u64::from(mat.match_seq_num) + 1)
        });

        // perhaps do something more useful with result
        // like saving it to a database for later query.
        for mat in matches {
            println!("Match: {:?}", mat);
        }

        tokio::time::sleep(Duration::from_millis(args.interval)).await;
    }
    Ok(())
}
