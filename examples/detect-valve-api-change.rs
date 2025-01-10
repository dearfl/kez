/// This is an simple example used to check Valve API updates.
/// The example will return a failure if any API is updated.
use clap::Parser;
use kez::{
    dota2::{
        get_match_history::MatchHistoryParameter,
        get_match_history_by_seq_num::MatchHistoryBySeqNumParameter,
    },
    Client, Error,
};

#[derive(Parser)]
pub struct Args {
    pub key: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = Client::new(args.key)?;

    // test get_heroes
    let result = client.get_heroes(()).await;
    if let Err(Error::DecodeError(err, content)) = result {
        anyhow::bail!("DecodeError: {err}\nContent: {content}");
    };

    // test get_match_history
    let result = client
        .get_match_history(MatchHistoryParameter::default())
        .await;
    let para = match result {
        Ok(history) => {
            let match_seq_num = history
                .matches
                .iter()
                .fold(0, |init, mat| std::cmp::max(init, mat.match_seq_num));
            MatchHistoryBySeqNumParameter::start_at(match_seq_num)
        }
        Err(Error::DecodeError(err, content)) => {
            anyhow::bail!("DecodeError: {err}\nContent: {content}");
        }
        Err(_) => MatchHistoryBySeqNumParameter::default(),
    };

    // test get_match_history_by_seq_num
    let result = client.get_match_history_by_seq_num(para).await;
    if let Err(Error::DecodeError(err, content)) = result {
        anyhow::bail!("DecodeError: {err}\nContent: {content}");
    };
    Ok(())
}
