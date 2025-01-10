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
    let mut matches = vec![];
    let client = Client::new(args.key)?;
    let mut start = args.start;
    let end = args.end;
    while start < end {
        println!("collecting matches starting from {}", start);
        let result = client.get_match_history_by_seq_num((start, 100)).await?;

        // update start match_seq_num
        start = result.matches.iter().fold(start + 1, |init, mat| {
            println!("match: {:?}", mat);
            std::cmp::max(init, mat.match_seq_num + 1)
        });

        // perhaps do something useful with result
        // like save it to a database for later query.
        matches.extend(result.matches);

        tokio::time::sleep(Duration::from_millis(args.interval)).await;
    }
    Ok(())
}
