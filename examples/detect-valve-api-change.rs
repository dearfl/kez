/// This is an simple example used to check Valve API updates.
/// The example will return a failure if any API is updated.
/// The feature deny-unknown-fields must be enabled for this example to work.
use clap::Parser;
use kez::{
    dota2::{
        get_match_history::MatchHistoryParameter, Ability, Engine, Hero, HeroId, Item, LeaveStatus,
        Lobby, MatchSeqNum, Mode,
    },
    Client, Error,
};

#[derive(Parser)]
pub struct Args {
    pub key: String,
}

fn check_hero(hero: Hero) -> Option<(u8, u8)> {
    use kez::dota2::hero::*;
    let raw = hero.into();
    match hero {
        Hero::Unknown((id, _)) if id != 0 => Some(raw),
        Hero::Antimage(facet) if matches!(facet, AntimageFacet::Unknown(_)) => Some(raw),

        Hero::Axe(facet) if matches!(facet, AxeFacet::Unknown(_)) => Some(raw),
        Hero::Bane(facet) if matches!(facet, BaneFacet::Unknown(_)) => Some(raw),
        Hero::Bloodseeker(facet) if matches!(facet, BloodseekerFacet::Unknown(_)) => Some(raw),
        Hero::CrystalMaiden(facet) if matches!(facet, CrystalMaidenFacet::Unknown(_)) => Some(raw),
        Hero::DrowRanger(facet) if matches!(facet, DrowRangerFacet::Unknown(_)) => Some(raw),
        Hero::Earthshaker(facet) if matches!(facet, EarthshakerFacet::Unknown(_)) => Some(raw),
        Hero::Juggernaut(facet) if matches!(facet, JuggernautFacet::Unknown(_)) => Some(raw),
        Hero::Mirana(facet) if matches!(facet, MiranaFacet::Unknown(_)) => Some(raw),
        Hero::Morphling(facet) if matches!(facet, MorphlingFacet::Unknown(_)) => Some(raw),
        Hero::Nevermore(facet) if matches!(facet, NevermoreFacet::Unknown(_)) => Some(raw),
        Hero::PhantomLancer(facet) if matches!(facet, PhantomLancerFacet::Unknown(_)) => Some(raw),
        Hero::Puck(facet) if matches!(facet, PuckFacet::Unknown(_)) => Some(raw),
        Hero::Pudge(facet) if matches!(facet, PudgeFacet::Unknown(_)) => Some(raw),
        Hero::Razor(facet) if matches!(facet, RazorFacet::Unknown(_)) => Some(raw),
        Hero::SandKing(facet) if matches!(facet, SandKingFacet::Unknown(_)) => Some(raw),
        Hero::StormSpirit(facet) if matches!(facet, StormSpiritFacet::Unknown(_)) => Some(raw),
        Hero::Sven(facet) if matches!(facet, SvenFacet::Unknown(_)) => Some(raw),
        Hero::Tiny(facet) if matches!(facet, TinyFacet::Unknown(_)) => Some(raw),
        Hero::Vengefulspirit(facet) if matches!(facet, VengefulspiritFacet::Unknown(_)) => {
            Some(raw)
        }
        Hero::Windrunner(facet) if matches!(facet, WindrunnerFacet::Unknown(_)) => Some(raw),
        Hero::Zuus(facet) if matches!(facet, ZuusFacet::Unknown(_)) => Some(raw),
        Hero::Kunkka(facet) if matches!(facet, KunkkaFacet::Unknown(_)) => Some(raw),
        Hero::Lina(facet) if matches!(facet, LinaFacet::Unknown(_)) => Some(raw),
        Hero::Lion(facet) if matches!(facet, LionFacet::Unknown(_)) => Some(raw),
        Hero::ShadowShaman(facet) if matches!(facet, ShadowShamanFacet::Unknown(_)) => Some(raw),
        Hero::Slardar(facet) if matches!(facet, SlardarFacet::Unknown(_)) => Some(raw),
        Hero::Tidehunter(facet) if matches!(facet, TidehunterFacet::Unknown(_)) => Some(raw),
        Hero::WitchDoctor(facet) if matches!(facet, WitchDoctorFacet::Unknown(_)) => Some(raw),
        Hero::Lich(facet) if matches!(facet, LichFacet::Unknown(_)) => Some(raw),
        Hero::Riki(facet) if matches!(facet, RikiFacet::Unknown(_)) => Some(raw),
        Hero::Enigma(facet) if matches!(facet, EnigmaFacet::Unknown(_)) => Some(raw),
        Hero::Tinker(facet) if matches!(facet, TinkerFacet::Unknown(_)) => Some(raw),
        Hero::Sniper(facet) if matches!(facet, SniperFacet::Unknown(_)) => Some(raw),
        Hero::Necrolyte(facet) if matches!(facet, NecrolyteFacet::Unknown(_)) => Some(raw),
        Hero::Warlock(facet) if matches!(facet, WarlockFacet::Unknown(_)) => Some(raw),
        Hero::Beastmaster(facet) if matches!(facet, BeastmasterFacet::Unknown(_)) => Some(raw),
        Hero::Queenofpain(facet) if matches!(facet, QueenofpainFacet::Unknown(_)) => Some(raw),
        Hero::Venomancer(facet) if matches!(facet, VenomancerFacet::Unknown(_)) => Some(raw),
        Hero::FacelessVoid(facet) if matches!(facet, FacelessVoidFacet::Unknown(_)) => Some(raw),
        Hero::SkeletonKing(facet) if matches!(facet, SkeletonKingFacet::Unknown(_)) => Some(raw),
        Hero::DeathProphet(facet) if matches!(facet, DeathProphetFacet::Unknown(_)) => Some(raw),
        Hero::PhantomAssassin(facet) if matches!(facet, PhantomAssassinFacet::Unknown(_)) => {
            Some(raw)
        }
        Hero::Pugna(facet) if matches!(facet, PugnaFacet::Unknown(_)) => Some(raw),
        Hero::TemplarAssassin(facet) if matches!(facet, TemplarAssassinFacet::Unknown(_)) => {
            Some(raw)
        }
        Hero::Viper(facet) if matches!(facet, ViperFacet::Unknown(_)) => Some(raw),
        Hero::Luna(facet) if matches!(facet, LunaFacet::Unknown(_)) => Some(raw),
        Hero::DragonKnight(facet) if matches!(facet, DragonKnightFacet::Unknown(_)) => Some(raw),
        Hero::Dazzle(facet) if matches!(facet, DazzleFacet::Unknown(_)) => Some(raw),
        Hero::Rattletrap(facet) if matches!(facet, RattletrapFacet::Unknown(_)) => Some(raw),
        Hero::Leshrac(facet) if matches!(facet, LeshracFacet::Unknown(_)) => Some(raw),
        Hero::Furion(facet) if matches!(facet, FurionFacet::Unknown(_)) => Some(raw),
        Hero::LifeStealer(facet) if matches!(facet, LifeStealerFacet::Unknown(_)) => Some(raw),
        Hero::DarkSeer(facet) if matches!(facet, DarkSeerFacet::Unknown(_)) => Some(raw),
        Hero::Clinkz(facet) if matches!(facet, ClinkzFacet::Unknown(_)) => Some(raw),
        Hero::Omniknight(facet) if matches!(facet, OmniknightFacet::Unknown(_)) => Some(raw),
        Hero::Enchantress(facet) if matches!(facet, EnchantressFacet::Unknown(_)) => Some(raw),
        Hero::Huskar(facet) if matches!(facet, HuskarFacet::Unknown(_)) => Some(raw),
        Hero::NightStalker(facet) if matches!(facet, NightStalkerFacet::Unknown(_)) => Some(raw),
        Hero::Broodmother(facet) if matches!(facet, BroodmotherFacet::Unknown(_)) => Some(raw),
        Hero::BountyHunter(facet) if matches!(facet, BountyHunterFacet::Unknown(_)) => Some(raw),
        Hero::Weaver(facet) if matches!(facet, WeaverFacet::Unknown(_)) => Some(raw),
        Hero::Jakiro(facet) if matches!(facet, JakiroFacet::Unknown(_)) => Some(raw),
        Hero::Batrider(facet) if matches!(facet, BatriderFacet::Unknown(_)) => Some(raw),
        Hero::Chen(facet) if matches!(facet, ChenFacet::Unknown(_)) => Some(raw),
        Hero::Spectre(facet) if matches!(facet, SpectreFacet::Unknown(_)) => Some(raw),
        Hero::AncientApparition(facet) if matches!(facet, AncientApparitionFacet::Unknown(_)) => {
            Some(raw)
        }
        Hero::DoomBringer(facet) if matches!(facet, DoomBringerFacet::Unknown(_)) => Some(raw),
        Hero::Ursa(facet) if matches!(facet, UrsaFacet::Unknown(_)) => Some(raw),
        Hero::SpiritBreaker(facet) if matches!(facet, SpiritBreakerFacet::Unknown(_)) => Some(raw),
        Hero::Gyrocopter(facet) if matches!(facet, GyrocopterFacet::Unknown(_)) => Some(raw),
        Hero::Alchemist(facet) if matches!(facet, AlchemistFacet::Unknown(_)) => Some(raw),
        Hero::Invoker(facet) if matches!(facet, InvokerFacet::Unknown(_)) => Some(raw),
        Hero::Silencer(facet) if matches!(facet, SilencerFacet::Unknown(_)) => Some(raw),
        Hero::ObsidianDestroyer(facet) if matches!(facet, ObsidianDestroyerFacet::Unknown(_)) => {
            Some(raw)
        }
        Hero::Lycan(facet) if matches!(facet, LycanFacet::Unknown(_)) => Some(raw),
        Hero::Brewmaster(facet) if matches!(facet, BrewmasterFacet::Unknown(_)) => Some(raw),
        Hero::ShadowDemon(facet) if matches!(facet, ShadowDemonFacet::Unknown(_)) => Some(raw),
        Hero::LoneDruid(facet) if matches!(facet, LoneDruidFacet::Unknown(_)) => Some(raw),
        Hero::ChaosKnight(facet) if matches!(facet, ChaosKnightFacet::Unknown(_)) => Some(raw),
        Hero::Meepo(facet) if matches!(facet, MeepoFacet::Unknown(_)) => Some(raw),
        Hero::Treant(facet) if matches!(facet, TreantFacet::Unknown(_)) => Some(raw),
        Hero::OgreMagi(facet) if matches!(facet, OgreMagiFacet::Unknown(_)) => Some(raw),
        Hero::Undying(facet) if matches!(facet, UndyingFacet::Unknown(_)) => Some(raw),
        Hero::Rubick(facet) if matches!(facet, RubickFacet::Unknown(_)) => Some(raw),
        Hero::Disruptor(facet) if matches!(facet, DisruptorFacet::Unknown(_)) => Some(raw),
        Hero::NyxAssassin(facet) if matches!(facet, NyxAssassinFacet::Unknown(_)) => Some(raw),
        Hero::NagaSiren(facet) if matches!(facet, NagaSirenFacet::Unknown(_)) => Some(raw),
        Hero::KeeperOfTheLight(facet) if matches!(facet, KeeperOfTheLightFacet::Unknown(_)) => {
            Some(raw)
        }
        Hero::Wisp(facet) if matches!(facet, WispFacet::Unknown(_)) => Some(raw),
        Hero::Visage(facet) if matches!(facet, VisageFacet::Unknown(_)) => Some(raw),
        Hero::Slark(facet) if matches!(facet, SlarkFacet::Unknown(_)) => Some(raw),
        Hero::Medusa(facet) if matches!(facet, MedusaFacet::Unknown(_)) => Some(raw),
        Hero::TrollWarlord(facet) if matches!(facet, TrollWarlordFacet::Unknown(_)) => Some(raw),
        Hero::Centaur(facet) if matches!(facet, CentaurFacet::Unknown(_)) => Some(raw),
        Hero::Magnataur(facet) if matches!(facet, MagnataurFacet::Unknown(_)) => Some(raw),
        Hero::Shredder(facet) if matches!(facet, ShredderFacet::Unknown(_)) => Some(raw),
        Hero::Bristleback(facet) if matches!(facet, BristlebackFacet::Unknown(_)) => Some(raw),
        Hero::Tusk(facet) if matches!(facet, TuskFacet::Unknown(_)) => Some(raw),
        Hero::SkywrathMage(facet) if matches!(facet, SkywrathMageFacet::Unknown(_)) => Some(raw),
        Hero::Abaddon(facet) if matches!(facet, AbaddonFacet::Unknown(_)) => Some(raw),
        Hero::ElderTitan(facet) if matches!(facet, ElderTitanFacet::Unknown(_)) => Some(raw),
        Hero::LegionCommander(facet) if matches!(facet, LegionCommanderFacet::Unknown(_)) => {
            Some(raw)
        }
        Hero::Techies(facet) if matches!(facet, TechiesFacet::Unknown(_)) => Some(raw),
        Hero::EmberSpirit(facet) if matches!(facet, EmberSpiritFacet::Unknown(_)) => Some(raw),
        Hero::EarthSpirit(facet) if matches!(facet, EarthSpiritFacet::Unknown(_)) => Some(raw),
        Hero::AbyssalUnderlord(facet) if matches!(facet, AbyssalUnderlordFacet::Unknown(_)) => {
            Some(raw)
        }
        Hero::Terrorblade(facet) if matches!(facet, TerrorbladeFacet::Unknown(_)) => Some(raw),
        Hero::Phoenix(facet) if matches!(facet, PhoenixFacet::Unknown(_)) => Some(raw),
        Hero::Oracle(facet) if matches!(facet, OracleFacet::Unknown(_)) => Some(raw),
        Hero::WinterWyvern(facet) if matches!(facet, WinterWyvernFacet::Unknown(_)) => Some(raw),
        Hero::ArcWarden(facet) if matches!(facet, ArcWardenFacet::Unknown(_)) => Some(raw),
        Hero::MonkeyKing(facet) if matches!(facet, MonkeyKingFacet::Unknown(_)) => Some(raw),
        Hero::DarkWillow(facet) if matches!(facet, DarkWillowFacet::Unknown(_)) => Some(raw),
        Hero::Pangolier(facet) if matches!(facet, PangolierFacet::Unknown(_)) => Some(raw),
        Hero::Grimstroke(facet) if matches!(facet, GrimstrokeFacet::Unknown(_)) => Some(raw),
        Hero::Hoodwink(facet) if matches!(facet, HoodwinkFacet::Unknown(_)) => Some(raw),
        Hero::VoidSpirit(facet) if matches!(facet, VoidSpiritFacet::Unknown(_)) => Some(raw),
        Hero::Snapfire(facet) if matches!(facet, SnapfireFacet::Unknown(_)) => Some(raw),
        Hero::Mars(facet) if matches!(facet, MarsFacet::Unknown(_)) => Some(raw),
        Hero::Ringmaster(facet) if matches!(facet, RingmasterFacet::Unknown(_)) => Some(raw),
        Hero::Dawnbreaker(facet) if matches!(facet, DawnbreakerFacet::Unknown(_)) => Some(raw),
        Hero::Marci(facet) if matches!(facet, MarciFacet::Unknown(_)) => Some(raw),
        Hero::PrimalBeast(facet) if matches!(facet, PrimalBeastFacet::Unknown(_)) => Some(raw),
        Hero::Muerta(facet) if matches!(facet, MuertaFacet::Unknown(_)) => Some(raw),
        Hero::Kez(facet) if matches!(facet, KezFacet::Unknown(_)) => Some(raw),
        _ => None,
    }
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
            let heroes: Vec<HeroId> = heroes
                .heroes
                .iter()
                .map(|hero| HeroId::from(hero.id))
                .collect();
            let unknown: Vec<_> = heroes
                .iter()
                .filter_map(|&hero| match hero {
                    HeroId::Unknown(id) => Some(id),
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
                    if let Some((id, facet)) = check_hero(player.hero) {
                        anyhow::bail!("Unknown hero or facet: {}, {}", id, facet);
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
