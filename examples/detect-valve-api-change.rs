/// This is an simple example used to check Valve API updates.
/// The example will return a failure if any API is updated.
/// The feature deny-unknown-fields must be enabled for this example to work.
use clap::Parser;
use kez::{
    dota2::{
        get_match_history::MatchHistoryParameter, Ability, Engine, Hero, Item, LeaveStatus, Lobby,
        MatchSeqNum, Mode,
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
        Err(err) => {
            println!("Error: {}", err);
            // we don't care about other errors like reqwest error
        }
    }

    // test get_match_history
    let result = client
        .get_match_history(MatchHistoryParameter::default())
        .await;
    let start_seq_num = match result {
        Ok(history) => {
            let match_seq_num = history
                .matches
                .iter()
                .fold(0, |init, mat| std::cmp::max(init, mat.match_seq_num));
            MatchSeqNum::from(match_seq_num)
        }
        Err(Error::DecodeError(err, content)) => {
            anyhow::bail!("DecodeError: {err}\nContent: {content}");
        }
        Err(err) => {
            println!("Error: {}", err);
            MatchSeqNum::default()
        }
    };

    // test history & get_match_history_by_seq_num
    let result = client.history(start_seq_num, 100).await;
    match result {
        Ok(matches) => {
            for mat in matches {
                for player in mat.players {
                    if let Hero::Unknown(id) = player.hero.0 {
                        if id != 0 {
                            // sometimes we will get hero_id = 0
                            anyhow::bail!("Unknown hero_id: {}", id);
                        }
                    }
                    let items = vec![
                        player.item_0,
                        player.item_1,
                        player.item_2,
                        player.item_3,
                        player.item_4,
                        player.item_5,
                        player.backpack_0,
                        player.backpack_1,
                        player.backpack_2,
                        player.item_neutral,
                    ];
                    for item in items {
                        if let Some(Item::Unknown(id)) = item {
                            anyhow::bail!("Unknown item_id: {}", id);
                        }
                    }
                    for upgrade in player.ability_upgrades {
                        if let Ability::Unknown(id) = upgrade.ability {
                            // there are always some ability constant we do not yet know?
                            // so we don't return an error here.
                            println!("Unknown ability id: {}, try checking https://opendota.com/matches/{}", id, u64::from(mat.match_id));
                        }
                    }

                    if let Some(LeaveStatus::Unknown(id)) = player.leave_status {
                        anyhow::bail!("Unknown leave_status: {}", id);
                    }
                }

                if let Lobby::Unknown(id) = mat.lobby_type {
                    anyhow::bail!("Unknown lobby_type: {}", id);
                }
                if let Mode::Unknown(id) = mat.mode {
                    anyhow::bail!("Unknown game_mode: {}", id);
                }
                if let Engine::Unknown(id) = mat.engine {
                    anyhow::bail!("Unknown engine: {}", id);
                }
            }
        }
        Err(Error::DecodeError(err, content)) => {
            anyhow::bail!("DecodeError: {err}\nContent: {content}");
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    };

    Ok(())
}
