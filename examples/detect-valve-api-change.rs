/// This is an simple example used to check Valve API updates.
/// The example will return a failure if any API is updated.
use clap::Parser;
use kez::{
    dota2::{
        get_match_history::MatchHistoryParameter,
        get_match_history_by_seq_num::MatchHistoryBySeqNumParameter, r#match::MatchSeqNum, Hero,
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
    // currently there are 126 unique heroes in dota2
    const N: usize = 126;

    // test get_heroes
    let result = client.get_heroes(()).await;
    match result {
        Ok(heroes) => {
            if heroes.heroes.len() != N {
                anyhow::bail!(
                    "Some new heroes have been added to dota2. We need to update this crate."
                );
            }
            let heroes: Vec<Hero> = heroes
                .heroes
                .iter()
                .map(|hero| Hero::from(hero.id))
                .collect();
            let unknown: Vec<_> = heroes
                .iter()
                .filter_map(|&hero| match hero {
                    Hero::Unknown(id) => Some(id),
                    _ => None,
                })
                .collect();
            if unknown.len() > 0 {
                anyhow::bail!(
                    "Unknown hero ids: {:?}. We need to update this crate.",
                    unknown
                );
            }
        }
        Err(Error::DecodeError(err, content)) => {
            anyhow::bail!("DecodeError: {err}\nContent: {content}");
        }
        Err(_) => {
            // we don't care about other errors like reqwest error
        }
    }

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
            MatchSeqNum::from(match_seq_num).into()
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
