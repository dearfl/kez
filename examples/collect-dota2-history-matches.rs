use clap::Parser;
use kez::Client;

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
        println!("collecting {}...", start);
        let result = client.get_match_history_by_seq_num((start, 100)).await?;
        start = result.matches.iter().fold(start + 1, |init, mat| {
            println!("match_id: {}", mat.match_id);
            std::cmp::max(init, mat.match_seq_num + 1)
        });
        // do something useful with result
        matches.extend(result.matches);
    }
    Ok(())
}
