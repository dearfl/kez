use reqwest::RequestBuilder;

use crate::{dota2::define_dota2_enum, Transform};

define_dota2_enum! {
    /// Available heros in current dota2
    /// # Example
    /// ```rust,no_run
    /// use kez::{dota2::{HeroId, get_heroes::Heroes}, Client};
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///   let client = Client::new("DUMMY_STEAM_API_KEY")?;
    ///   let heroes = client.get_heroes(()).await?;
    ///   let heroes: Vec<HeroId> = heroes.heroes.iter().map(|hero| HeroId::from(hero.id)).collect();
    ///   println!("{:?}", heroes);
    ///   Ok(())
    /// }
    /// ```
    pub enum HeroId : u8 {
        Antimage = 1,
        Axe = 2,
        Bane = 3,
        Bloodseeker = 4,
        CrystalMaiden = 5,
        DrowRanger = 6,
        Earthshaker = 7,
        Juggernaut = 8,
        Mirana = 9,
        Morphling = 10,
        Nevermore = 11,
        PhantomLancer = 12,
        Puck = 13,
        Pudge = 14,
        Razor = 15,
        SandKing = 16,
        StormSpirit = 17,
        Sven = 18,
        Tiny = 19,
        Vengefulspirit = 20,
        Windrunner = 21,
        Zuus = 22,
        Kunkka = 23,
        Lina = 25,
        Lion = 26,
        ShadowShaman = 27,
        Slardar = 28,
        Tidehunter = 29,
        WitchDoctor = 30,
        Lich = 31,
        Riki = 32,
        Enigma = 33,
        Tinker = 34,
        Sniper = 35,
        Necrolyte = 36,
        Warlock = 37,
        Beastmaster = 38,
        Queenofpain = 39,
        Venomancer = 40,
        FacelessVoid = 41,
        SkeletonKing = 42,
        DeathProphet = 43,
        PhantomAssassin = 44,
        Pugna = 45,
        TemplarAssassin = 46,
        Viper = 47,
        Luna = 48,
        DragonKnight = 49,
        Dazzle = 50,
        Rattletrap = 51,
        Leshrac = 52,
        Furion = 53,
        LifeStealer = 54,
        DarkSeer = 55,
        Clinkz = 56,
        Omniknight = 57,
        Enchantress = 58,
        Huskar = 59,
        NightStalker = 60,
        Broodmother = 61,
        BountyHunter = 62,
        Weaver = 63,
        Jakiro = 64,
        Batrider = 65,
        Chen = 66,
        Spectre = 67,
        AncientApparition = 68,
        DoomBringer = 69,
        Ursa = 70,
        SpiritBreaker = 71,
        Gyrocopter = 72,
        Alchemist = 73,
        Invoker = 74,
        Silencer = 75,
        ObsidianDestroyer = 76,
        Lycan = 77,
        Brewmaster = 78,
        ShadowDemon = 79,
        LoneDruid = 80,
        ChaosKnight = 81,
        Meepo = 82,
        Treant = 83,
        OgreMagi = 84,
        Undying = 85,
        Rubick = 86,
        Disruptor = 87,
        NyxAssassin = 88,
        NagaSiren = 89,
        KeeperOfTheLight = 90,
        Wisp = 91,
        Visage = 92,
        Slark = 93,
        Medusa = 94,
        TrollWarlord = 95,
        Centaur = 96,
        Magnataur = 97,
        Shredder = 98,
        Bristleback = 99,
        Tusk = 100,
        SkywrathMage = 101,
        Abaddon = 102,
        ElderTitan = 103,
        LegionCommander = 104,
        Techies = 105,
        EmberSpirit = 106,
        EarthSpirit = 107,
        AbyssalUnderlord = 108,
        Terrorblade = 109,
        Phoenix = 110,
        Oracle = 111,
        WinterWyvern = 112,
        ArcWarden = 113,
        MonkeyKing = 114,
        DarkWillow = 119,
        Pangolier = 120,
        Grimstroke = 121,
        Hoodwink = 123,
        VoidSpirit = 126,
        Snapfire = 128,
        Mars = 129,
        Ringmaster = 131,
        Dawnbreaker = 135,
        Marci = 136,
        PrimalBeast = 137,
        Muerta = 138,
        Kez = 145,
    }
}

impl Transform<HeroId> for RequestBuilder {
    fn transform(self, value: HeroId) -> Self {
        self.query(&[("hero_id", u8::from(value))])
    }
}

// TODO: maybe use another macro for heroes definitions
define_dota2_enum! {
    pub enum AntimageFacet : u8 {
        MagebanesMirror = 1,
        ManaThirst = 2,
    }
}

define_dota2_enum! {
    pub enum AxeFacet : u8 {
        OneManArmy = 1,
        CallOut = 2,
    }
}

define_dota2_enum! {
    pub enum BaneFacet : u8 {
        DreamStalker = 1,
        Sleepwalk = 2,
    }
}

define_dota2_enum! {
    pub enum BloodseekerFacet : u8 {
        ArterialSpray = 1,
        Bloodrush = 2,
    }
}

define_dota2_enum! {
    pub enum CrystalMaidenFacet : u8 {
        FrozenExpanse = 1,
        ColdComfort = 2,
    }
}

define_dota2_enum! {
    pub enum DrowRangerFacet : u8 {
        HighGround = 1,
        Sidestep = 2,
    }
}

define_dota2_enum! {
    pub enum EarthshakerFacet : u8 {
        TectonicBuildup = 1,
        Slugger = 2,
    }
}

define_dota2_enum! {
    pub enum JuggernautFacet : u8 {
        Bladestorm = 1,
        Agigain = 2,
    }
}

define_dota2_enum! {
    pub enum MiranaFacet : u8 {
        Moonlight = 1,
        Sunlight = 2,
    }
}

define_dota2_enum! {
    pub enum MorphlingFacet : u8 {
        Agi = 1,
        Str = 2,
    }
}

define_dota2_enum! {
    pub enum NevermoreFacet : u8 {
        LastingPresence = 1,
        Shadowmire = 2,
    }
}

define_dota2_enum! {
    pub enum PhantomLancerFacet : u8 {
        Convergence = 1,
        Divergence = 2,
    }
}

define_dota2_enum! {
    pub enum PuckFacet : u8 {
        JostlingRift = 1,
        Curveball = 2,
    }
}

define_dota2_enum! {
    pub enum PudgeFacet : u8 {
        FreshMeat = 1,
        FlayersHook = 2,
    }
}

define_dota2_enum! {
    pub enum RazorFacet : u8 {
        Thunderhead = 1,
        Spellamp = 2,
    }
}

define_dota2_enum! {
    pub enum SandKingFacet : u8 {
        Sandshroud = 1,
        DustDevil = 2,
    }
}

define_dota2_enum! {
    pub enum StormSpiritFacet : u8 {
        ShockCollar = 1,
        StaticSlide = 2,
    }
}

define_dota2_enum! {
    pub enum SvenFacet : u8 {
        HeavyPlate = 1,
        Strscaling = 2,
    }
}

define_dota2_enum! {
    pub enum TinyFacet : u8 {
        CrashLanding = 1,
        Insurmountable = 2,
    }
}

define_dota2_enum! {
    pub enum VengefulspiritFacet : u8 {
        AvengingMissile = 1,
        Melee = 2,
    }
}

define_dota2_enum! {
    pub enum WindrunnerFacet : u8 {
        Focusfire = 2,
        Whirlwind = 3,
    }
}

define_dota2_enum! {
    pub enum ZuusFacet : u8 {
        Livewire = 1,
        DivineRampage = 2,
    }
}

define_dota2_enum! {
    pub enum KunkkaFacet : u8 {
        HighTide = 1,
        Grog = 2,
    }
}

define_dota2_enum! {
    pub enum LinaFacet : u8 {
        Supercharge = 1,
        Dot = 2,
    }
}

define_dota2_enum! {
    pub enum LionFacet : u8 {
        EssenceEater = 1,
        FistOfDeath = 2,
    }
}

define_dota2_enum! {
    pub enum ShadowShamanFacet : u8 {
        ClusterCluck = 1,
        MassiveSerpentWard = 2,
    }
}

define_dota2_enum! {
    pub enum SlardarFacet : u8 {
        LegDay = 1,
        Brineguard = 2,
    }
}

define_dota2_enum! {
    pub enum TidehunterFacet : u8 {
        KrakenSwell = 1,
        Sizescale = 2,
    }
}

define_dota2_enum! {
    pub enum WitchDoctorFacet : u8 {
        Headhunter = 1,
        VoodooFesteration = 2,
        CleftDeath = 3,
    }
}

define_dota2_enum! {
    pub enum LichFacet : u8 {
        Frostbound = 1,
        GrowingCold = 2,
    }
}

define_dota2_enum! {
    pub enum RikiFacet : u8 {
        ContractKiller = 1,
        Exterminator = 2,
    }
}

define_dota2_enum! {
    pub enum EnigmaFacet : u8 {
        Gravity = 1,
        Fragment = 2,
    }
}

define_dota2_enum! {
    pub enum TinkerFacet : u8 {
        RepairBots = 1,
        Translocator = 2,
    }
}

define_dota2_enum! {
    pub enum SniperFacet : u8 {
        GhillieSuit = 1,
        Scattershot = 2,
    }
}

define_dota2_enum! {
    pub enum NecrolyteFacet : u8 {
        ProfanePotency = 1,
        RapidDecay = 2,
    }
}

define_dota2_enum! {
    pub enum WarlockFacet : u8 {
        Golem = 1,
        Grimoire = 2,
    }
}

define_dota2_enum! {
    pub enum BeastmasterFacet : u8 {
        WildHunt = 1,
        BeastMode = 2,
    }
}

define_dota2_enum! {
    pub enum QueenofpainFacet : u8 {
        Lifesteal = 1,
        Selfdmg = 2,
    }
}

define_dota2_enum! {
    pub enum VenomancerFacet : u8 {
        PatientZero = 1,
        PlagueCarrier = 2,
    }
}

define_dota2_enum! {
    pub enum FacelessVoidFacet : u8 {
        Chronosphere = 2,
        TimeZone = 3,
    }
}

define_dota2_enum! {
    pub enum SkeletonKingFacet : u8 {
        FacetBoneGuard = 1,
        FacetCursedBlade = 2,
    }
}

define_dota2_enum! {
    pub enum DeathProphetFacet : u8 {
        Suppress = 1,
        Ghosts = 2,
        DelayedDamage = 3,
    }
}

define_dota2_enum! {
    pub enum PhantomAssassinFacet : u8 {
        VeiledOne = 1,
        Methodical = 2,
    }
}

define_dota2_enum! {
    pub enum PugnaFacet : u8 {
        SiphoningWard = 1,
        RewardsOfRuin = 2,
    }
}

define_dota2_enum! {
    pub enum TemplarAssassinFacet : u8 {
        Voidblades = 1,
        Refractor = 2,
    }
}

define_dota2_enum! {
    pub enum ViperFacet : u8 {
        PoisonBurst = 1,
        CausticBath = 2,
    }
}

define_dota2_enum! {
    pub enum LunaFacet : u8 {
        Moonshield = 2,
        Moonstorm = 3,
    }
}

define_dota2_enum! {
    pub enum DragonKnightFacet : u8 {
        FireDragon = 1,
        CorrosiveDragon = 2,
        FrostDragon = 3,
    }
}

define_dota2_enum! {
    pub enum DazzleFacet : u8 {
        FacetNothlBoon = 1,
        PoisonBloom = 2,
    }
}

define_dota2_enum! {
    pub enum RattletrapFacet : u8 {
        Hookup = 1,
        ExpandedArmature = 2,
    }
}

define_dota2_enum! {
    pub enum LeshracFacet : u8 {
        AttacksMana = 1,
        Misanthropy = 2,
    }
}

define_dota2_enum! {
    pub enum FurionFacet : u8 {
        SoothingSaplings = 1,
        IronwoodTreant = 2,
    }
}

define_dota2_enum! {
    pub enum LifeStealerFacet : u8 {
        Rage = 2,
        RageDispell = 3,
    }
}

define_dota2_enum! {
    pub enum DarkSeerFacet : u8 {
        Atkspd = 1,
        Movespd = 2,
    }
}

define_dota2_enum! {
    pub enum ClinkzFacet : u8 {
        SuppressiveFire = 1,
        EngulfingStep = 2,
    }
}

define_dota2_enum! {
    pub enum OmniknightFacet : u8 {
        Omnipresent = 1,
        Dmgheals = 2,
    }
}

define_dota2_enum! {
    pub enum EnchantressFacet : u8 {
        OverprotectiveWisps = 1,
        Spellbound = 2,
    }
}

define_dota2_enum! {
    pub enum HuskarFacet : u8 {
        Bloodbath = 1,
        NothlTransfusion = 2,
        NothlConflagration = 3,
    }
}

define_dota2_enum! {
    pub enum NightStalkerFacet : u8 {
        BlindingVoid = 1,
        Dayswap = 2,
    }
}

define_dota2_enum! {
    pub enum BroodmotherFacet : u8 {
        NecroticWebs = 1,
        FeedingFrenzy = 2,
    }
}

define_dota2_enum! {
    pub enum BountyHunterFacet : u8 {
        Shuriken = 1,
        Mugging = 2,
    }
}

define_dota2_enum! {
    pub enum WeaverFacet : u8 {
        Skitterstep = 1,
        Hivemind = 2,
    }
}

define_dota2_enum! {
    pub enum JakiroFacet : u8 {
        Fire = 1,
        Ice = 2,
    }
}

define_dota2_enum! {
    pub enum BatriderFacet : u8 {
        BuffOnDisplacement = 1,
        Arsonist = 2,
    }
}

define_dota2_enum! {
    pub enum ChenFacet : u8 {
        CentaurConvert = 1,
        WolfConvert = 2,
        HellbearConvert = 3,
        TrollConvert = 4,
        SatyrConvert = 5,
    }
}

define_dota2_enum! {
    pub enum SpectreFacet : u8 {
        Forsaken = 1,
        TwistTheKnife = 2,
    }
}

define_dota2_enum! {
    pub enum AncientApparitionFacet : u8 {
        BoneChill = 1,
        Exposure = 2,
    }
}

define_dota2_enum! {
    pub enum DoomBringerFacet : u8 {
        Gluttony = 1,
        BoostSelling = 2,
        ImpendingDoom = 3,
    }
}

define_dota2_enum! {
    pub enum UrsaFacet : u8 {
        GrudgeBearer = 1,
        DebuffReduce = 2,
    }
}

define_dota2_enum! {
    pub enum SpiritBreakerFacet : u8 {
        BullRush = 1,
        Imbalanced = 2,
    }
}

define_dota2_enum! {
    pub enum GyrocopterFacet : u8 {
        SecondaryStrikes = 1,
        Afterburner = 2,
    }
}

define_dota2_enum! {
    pub enum AlchemistFacet : u8 {
        SeedMoney = 1,
        Mixologist = 2,
    }
}

define_dota2_enum! {
    pub enum InvokerFacet : u8 {
        Agnostic = 1,
        Elitist = 2,
    }
}

define_dota2_enum! {
    pub enum SilencerFacet : u8 {
        Irrepressible = 1,
        ReverberatingSilence = 2,
    }
}

define_dota2_enum! {
    pub enum ObsidianDestroyerFacet : u8 {
        ObsidianDecimator = 1,
        OverwhelmingDevourer = 2,
    }
}

define_dota2_enum! {
    pub enum LycanFacet : u8 {
        PackLeader = 1,
        SpiritWolves = 2,
        AlphaWolves = 3,
    }
}

define_dota2_enum! {
    pub enum BrewmasterFacet : u8 {
        RollOutTheBarrel = 1,
        DrunkenMaster = 2,
    }
}

define_dota2_enum! {
    pub enum ShadowDemonFacet : u8 {
        Promulgate = 1,
        FacetSoulMastery = 2,
    }
}

define_dota2_enum! {
    pub enum LoneDruidFacet : u8 {
        BearWithMe = 1,
        Unbearable = 2,
        BearNecessities = 3,
    }
}

define_dota2_enum! {
    pub enum ChaosKnightFacet : u8 {
        StrongIllusions = 1,
        Irrationality = 2,
    }
}

define_dota2_enum! {
    pub enum MeepoFacet : u8 {
        MoreMeepo = 1,
        PackRat = 2,
    }
}

define_dota2_enum! {
    pub enum TreantFacet : u8 {
        PrimevalPower = 1,
        Sapling = 2,
    }
}

define_dota2_enum! {
    pub enum OgreMagiFacet : u8 {
        FatChance = 1,
        LearningCurve = 2,
    }
}

define_dota2_enum! {
    pub enum UndyingFacet : u8 {
        RottingMitts = 1,
        Ripped = 2,
    }
}

define_dota2_enum! {
    pub enum RubickFacet : u8 {
        FrugalFilch = 1,
        ArcaneAccumulation = 2,
    }
}

define_dota2_enum! {
    pub enum DisruptorFacet : u8 {
        Thunderstorm = 1,
        LineWalls = 2,
    }
}

define_dota2_enum! {
    pub enum NyxAssassinFacet : u8 {
        BurnMana = 1,
        Scuttle = 2,
    }
}

define_dota2_enum! {
    pub enum NagaSirenFacet : u8 {
        PassiveRiptide = 1,
        ActiveRiptide = 2,
    }
}

define_dota2_enum! {
    pub enum KeeperOfTheLightFacet : u8 {
        FacetSolarBind = 1,
        FacetRecall = 2,
    }
}

define_dota2_enum! {
    pub enum WispFacet : u8 {
        Kritzkrieg = 1,
        Medigun = 2,
    }
}

define_dota2_enum! {
    pub enum VisageFacet : u8 {
        Sepulchre = 1,
        FaithfulFollowers = 2,
        GoldAssumption = 3,
    }
}

define_dota2_enum! {
    pub enum SlarkFacet : u8 {
        LeechingLeash = 1,
        DarkReefRenegade = 2,
    }
}

define_dota2_enum! {
    pub enum MedusaFacet : u8 {
        Engorged = 1,
        SlowAttacks = 3,
    }
}

define_dota2_enum! {
    pub enum TrollWarlordFacet : u8 {
        Insensitive = 1,
        BadInfluence = 2,
    }
}

define_dota2_enum! {
    pub enum CentaurFacet : u8 {
        CounterStrike = 1,
        Horsepower = 2,
    }
}

define_dota2_enum! {
    pub enum MagnataurFacet : u8 {
        ReversePolarity = 2,
        ReverseReversePolarity = 3,
    }
}

define_dota2_enum! {
    pub enum ShredderFacet : u8 {
        Shredder = 1,
        SecondChakram = 2,
    }
}

define_dota2_enum! {
    pub enum BristlebackFacet : u8 {
        Berserk = 1,
        SnotRocket = 2,
        SeeingRed = 3,
    }
}

define_dota2_enum! {
    pub enum TuskFacet : u8 {
        FacetTagTeam = 1,
        FacetFistBump = 2,
    }
}

define_dota2_enum! {
    pub enum SkywrathMageFacet : u8 {
        Shield = 1,
        Staff = 2,
    }
}

define_dota2_enum! {
    pub enum AbaddonFacet : u8 {
        DeathDude = 1,
        MephiticShroud = 2,
    }
}

define_dota2_enum! {
    pub enum ElderTitanFacet : u8 {
        Deconstruction = 1,
        BoostAtkspd = 2,
    }
}

define_dota2_enum! {
    pub enum LegionCommanderFacet : u8 {
        StonehallPlate = 1,
        SpoilsOfWar = 2,
    }
}

define_dota2_enum! {
    pub enum TechiesFacet : u8 {
        AtkRange = 1,
        SpleensSecretSauce = 2,
        Backpack = 3,
    }
}

define_dota2_enum! {
    pub enum EmberSpiritFacet : u8 {
        DoubleImpact = 1,
        ChainGang = 2,
    }
}

define_dota2_enum! {
    pub enum EarthSpiritFacet : u8 {
        Resonance = 1,
        SteppingStone = 2,
        ReadyToRoll = 3,
    }
}

define_dota2_enum! {
    pub enum AbyssalUnderlordFacet : u8 {
        DemonsReach = 1,
        Summons = 2,
    }
}

define_dota2_enum! {
    pub enum TerrorbladeFacet : u8 {
        Condemned = 1,
        SoulFragment = 2,
    }
}

define_dota2_enum! {
    pub enum PhoenixFacet : u8 {
        FacetImmolate = 1,
        Hotspot = 2,
    }
}

define_dota2_enum! {
    pub enum OracleFacet : u8 {
        FacetDmg = 1,
        FacetHeal = 2,
    }
}

define_dota2_enum! {
    pub enum WinterWyvernFacet : u8 {
        HealMana = 1,
        AtkRange = 2,
    }
}

define_dota2_enum! {
    pub enum ArcWardenFacet : u8 {
        Order = 1,
        Disorder = 2,
    }
}

define_dota2_enum! {
    pub enum MonkeyKingFacet : u8 {
        WukongsFaithful = 1,
        SimianStride = 2,
    }
}

define_dota2_enum! {
    pub enum DarkWillowFacet : u8 {
        ThrowingShade = 1,
        ThornyThicket = 2,
    }
}

define_dota2_enum! {
    pub enum PangolierFacet : u8 {
        DoubleJump = 1,
        Thunderbolt = 2,
    }
}

define_dota2_enum! {
    pub enum GrimstrokeFacet : u8 {
        Inkstigate = 1,
        FineArt = 2,
    }
}

define_dota2_enum! {
    pub enum HoodwinkFacet : u8 {
        Hunter = 1,
        TreebounceTrickshot = 2,
    }
}

define_dota2_enum! {
    pub enum VoidSpiritFacet : u8 {
        Sanctuary = 1,
        AetherArtifice = 3,
    }
}

define_dota2_enum! {
    pub enum SnapfireFacet : u8 {
        RicochetIi = 1,
        FullBore = 2,
    }
}

define_dota2_enum! {
    pub enum MarsFacet : u8 {
        VictoryFeast = 1,
        Arena = 2,
    }
}

define_dota2_enum! {
    pub enum RingmasterFacet : u8 {
        Default = 1,
    }
}

define_dota2_enum! {
    pub enum DawnbreakerFacet : u8 {
        SolarCharged = 1,
        GleamingHammer = 2,
    }
}

define_dota2_enum! {
    pub enum MarciFacet : u8 {
        Sidekick = 1,
        Bodyguard = 2,
    }
}

define_dota2_enum! {
    pub enum PrimalBeastFacet : u8 {
        RompNStomp = 1,
        Ferocity = 2,
    }
}

define_dota2_enum! {
    pub enum MuertaFacet : u8 {
        DanceOfTheDead = 1,
        Ofrenda = 2,
    }
}

define_dota2_enum! {
    pub enum KezFacet : u8 {
        Default = 1,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Hero {
    Unknown((u8, u8)) = u8::MAX,
    Antimage(AntimageFacet) = 1,
    Axe(AxeFacet) = 2,
    Bane(BaneFacet) = 3,
    Bloodseeker(BloodseekerFacet) = 4,
    CrystalMaiden(CrystalMaidenFacet) = 5,
    DrowRanger(DrowRangerFacet) = 6,
    Earthshaker(EarthshakerFacet) = 7,
    Juggernaut(JuggernautFacet) = 8,
    Mirana(MiranaFacet) = 9,
    Morphling(MorphlingFacet) = 10,
    Nevermore(NevermoreFacet) = 11,
    PhantomLancer(PhantomLancerFacet) = 12,
    Puck(PuckFacet) = 13,
    Pudge(PudgeFacet) = 14,
    Razor(RazorFacet) = 15,
    SandKing(SandKingFacet) = 16,
    StormSpirit(StormSpiritFacet) = 17,
    Sven(SvenFacet) = 18,
    Tiny(TinyFacet) = 19,
    Vengefulspirit(VengefulspiritFacet) = 20,
    Windrunner(WindrunnerFacet) = 21,
    Zuus(ZuusFacet) = 22,
    Kunkka(KunkkaFacet) = 23,
    Lina(LinaFacet) = 25,
    Lion(LionFacet) = 26,
    ShadowShaman(ShadowShamanFacet) = 27,
    Slardar(SlardarFacet) = 28,
    Tidehunter(TidehunterFacet) = 29,
    WitchDoctor(WitchDoctorFacet) = 30,
    Lich(LichFacet) = 31,
    Riki(RikiFacet) = 32,
    Enigma(EnigmaFacet) = 33,
    Tinker(TinkerFacet) = 34,
    Sniper(SniperFacet) = 35,
    Necrolyte(NecrolyteFacet) = 36,
    Warlock(WarlockFacet) = 37,
    Beastmaster(BeastmasterFacet) = 38,
    Queenofpain(QueenofpainFacet) = 39,
    Venomancer(VenomancerFacet) = 40,
    FacelessVoid(FacelessVoidFacet) = 41,
    SkeletonKing(SkeletonKingFacet) = 42,
    DeathProphet(DeathProphetFacet) = 43,
    PhantomAssassin(PhantomAssassinFacet) = 44,
    Pugna(PugnaFacet) = 45,
    TemplarAssassin(TemplarAssassinFacet) = 46,
    Viper(ViperFacet) = 47,
    Luna(LunaFacet) = 48,
    DragonKnight(DragonKnightFacet) = 49,
    Dazzle(DazzleFacet) = 50,
    Rattletrap(RattletrapFacet) = 51,
    Leshrac(LeshracFacet) = 52,
    Furion(FurionFacet) = 53,
    LifeStealer(LifeStealerFacet) = 54,
    DarkSeer(DarkSeerFacet) = 55,
    Clinkz(ClinkzFacet) = 56,
    Omniknight(OmniknightFacet) = 57,
    Enchantress(EnchantressFacet) = 58,
    Huskar(HuskarFacet) = 59,
    NightStalker(NightStalkerFacet) = 60,
    Broodmother(BroodmotherFacet) = 61,
    BountyHunter(BountyHunterFacet) = 62,
    Weaver(WeaverFacet) = 63,
    Jakiro(JakiroFacet) = 64,
    Batrider(BatriderFacet) = 65,
    Chen(ChenFacet) = 66,
    Spectre(SpectreFacet) = 67,
    AncientApparition(AncientApparitionFacet) = 68,
    DoomBringer(DoomBringerFacet) = 69,
    Ursa(UrsaFacet) = 70,
    SpiritBreaker(SpiritBreakerFacet) = 71,
    Gyrocopter(GyrocopterFacet) = 72,
    Alchemist(AlchemistFacet) = 73,
    Invoker(InvokerFacet) = 74,
    Silencer(SilencerFacet) = 75,
    ObsidianDestroyer(ObsidianDestroyerFacet) = 76,
    Lycan(LycanFacet) = 77,
    Brewmaster(BrewmasterFacet) = 78,
    ShadowDemon(ShadowDemonFacet) = 79,
    LoneDruid(LoneDruidFacet) = 80,
    ChaosKnight(ChaosKnightFacet) = 81,
    Meepo(MeepoFacet) = 82,
    Treant(TreantFacet) = 83,
    OgreMagi(OgreMagiFacet) = 84,
    Undying(UndyingFacet) = 85,
    Rubick(RubickFacet) = 86,
    Disruptor(DisruptorFacet) = 87,
    NyxAssassin(NyxAssassinFacet) = 88,
    NagaSiren(NagaSirenFacet) = 89,
    KeeperOfTheLight(KeeperOfTheLightFacet) = 90,
    Wisp(WispFacet) = 91,
    Visage(VisageFacet) = 92,
    Slark(SlarkFacet) = 93,
    Medusa(MedusaFacet) = 94,
    TrollWarlord(TrollWarlordFacet) = 95,
    Centaur(CentaurFacet) = 96,
    Magnataur(MagnataurFacet) = 97,
    Shredder(ShredderFacet) = 98,
    Bristleback(BristlebackFacet) = 99,
    Tusk(TuskFacet) = 100,
    SkywrathMage(SkywrathMageFacet) = 101,
    Abaddon(AbaddonFacet) = 102,
    ElderTitan(ElderTitanFacet) = 103,
    LegionCommander(LegionCommanderFacet) = 104,
    Techies(TechiesFacet) = 105,
    EmberSpirit(EmberSpiritFacet) = 106,
    EarthSpirit(EarthSpiritFacet) = 107,
    AbyssalUnderlord(AbyssalUnderlordFacet) = 108,
    Terrorblade(TerrorbladeFacet) = 109,
    Phoenix(PhoenixFacet) = 110,
    Oracle(OracleFacet) = 111,
    WinterWyvern(WinterWyvernFacet) = 112,
    ArcWarden(ArcWardenFacet) = 113,
    MonkeyKing(MonkeyKingFacet) = 114,
    DarkWillow(DarkWillowFacet) = 119,
    Pangolier(PangolierFacet) = 120,
    Grimstroke(GrimstrokeFacet) = 121,
    Hoodwink(HoodwinkFacet) = 123,
    VoidSpirit(VoidSpiritFacet) = 126,
    Snapfire(SnapfireFacet) = 128,
    Mars(MarsFacet) = 129,
    Ringmaster(RingmasterFacet) = 131,
    Dawnbreaker(DawnbreakerFacet) = 135,
    Marci(MarciFacet) = 136,
    PrimalBeast(PrimalBeastFacet) = 137,
    Muerta(MuertaFacet) = 138,
    Kez(KezFacet) = 145,
}

impl Default for Hero {
    fn default() -> Self {
        Self::Unknown((u8::MIN, u8::MIN))
    }
}

impl From<(u8, u8)> for Hero {
    fn from((id, facet): (u8, u8)) -> Self {
        match id {
            1 => Self::Antimage(AntimageFacet::from(facet)),
            2 => Self::Axe(AxeFacet::from(facet)),
            3 => Self::Bane(BaneFacet::from(facet)),
            4 => Self::Bloodseeker(BloodseekerFacet::from(facet)),
            5 => Self::CrystalMaiden(CrystalMaidenFacet::from(facet)),
            6 => Self::DrowRanger(DrowRangerFacet::from(facet)),
            7 => Self::Earthshaker(EarthshakerFacet::from(facet)),
            8 => Self::Juggernaut(JuggernautFacet::from(facet)),
            9 => Self::Mirana(MiranaFacet::from(facet)),
            10 => Self::Morphling(MorphlingFacet::from(facet)),
            11 => Self::Nevermore(NevermoreFacet::from(facet)),
            12 => Self::PhantomLancer(PhantomLancerFacet::from(facet)),
            13 => Self::Puck(PuckFacet::from(facet)),
            14 => Self::Pudge(PudgeFacet::from(facet)),
            15 => Self::Razor(RazorFacet::from(facet)),
            16 => Self::SandKing(SandKingFacet::from(facet)),
            17 => Self::StormSpirit(StormSpiritFacet::from(facet)),
            18 => Self::Sven(SvenFacet::from(facet)),
            19 => Self::Tiny(TinyFacet::from(facet)),
            20 => Self::Vengefulspirit(VengefulspiritFacet::from(facet)),
            21 => Self::Windrunner(WindrunnerFacet::from(facet)),
            22 => Self::Zuus(ZuusFacet::from(facet)),
            23 => Self::Kunkka(KunkkaFacet::from(facet)),
            25 => Self::Lina(LinaFacet::from(facet)),
            26 => Self::Lion(LionFacet::from(facet)),
            27 => Self::ShadowShaman(ShadowShamanFacet::from(facet)),
            28 => Self::Slardar(SlardarFacet::from(facet)),
            29 => Self::Tidehunter(TidehunterFacet::from(facet)),
            30 => Self::WitchDoctor(WitchDoctorFacet::from(facet)),
            31 => Self::Lich(LichFacet::from(facet)),
            32 => Self::Riki(RikiFacet::from(facet)),
            33 => Self::Enigma(EnigmaFacet::from(facet)),
            34 => Self::Tinker(TinkerFacet::from(facet)),
            35 => Self::Sniper(SniperFacet::from(facet)),
            36 => Self::Necrolyte(NecrolyteFacet::from(facet)),
            37 => Self::Warlock(WarlockFacet::from(facet)),
            38 => Self::Beastmaster(BeastmasterFacet::from(facet)),
            39 => Self::Queenofpain(QueenofpainFacet::from(facet)),
            40 => Self::Venomancer(VenomancerFacet::from(facet)),
            41 => Self::FacelessVoid(FacelessVoidFacet::from(facet)),
            42 => Self::SkeletonKing(SkeletonKingFacet::from(facet)),
            43 => Self::DeathProphet(DeathProphetFacet::from(facet)),
            44 => Self::PhantomAssassin(PhantomAssassinFacet::from(facet)),
            45 => Self::Pugna(PugnaFacet::from(facet)),
            46 => Self::TemplarAssassin(TemplarAssassinFacet::from(facet)),
            47 => Self::Viper(ViperFacet::from(facet)),
            48 => Self::Luna(LunaFacet::from(facet)),
            49 => Self::DragonKnight(DragonKnightFacet::from(facet)),
            50 => Self::Dazzle(DazzleFacet::from(facet)),
            51 => Self::Rattletrap(RattletrapFacet::from(facet)),
            52 => Self::Leshrac(LeshracFacet::from(facet)),
            53 => Self::Furion(FurionFacet::from(facet)),
            54 => Self::LifeStealer(LifeStealerFacet::from(facet)),
            55 => Self::DarkSeer(DarkSeerFacet::from(facet)),
            56 => Self::Clinkz(ClinkzFacet::from(facet)),
            57 => Self::Omniknight(OmniknightFacet::from(facet)),
            58 => Self::Enchantress(EnchantressFacet::from(facet)),
            59 => Self::Huskar(HuskarFacet::from(facet)),
            60 => Self::NightStalker(NightStalkerFacet::from(facet)),
            61 => Self::Broodmother(BroodmotherFacet::from(facet)),
            62 => Self::BountyHunter(BountyHunterFacet::from(facet)),
            63 => Self::Weaver(WeaverFacet::from(facet)),
            64 => Self::Jakiro(JakiroFacet::from(facet)),
            65 => Self::Batrider(BatriderFacet::from(facet)),
            66 => Self::Chen(ChenFacet::from(facet)),
            67 => Self::Spectre(SpectreFacet::from(facet)),
            68 => Self::AncientApparition(AncientApparitionFacet::from(facet)),
            69 => Self::DoomBringer(DoomBringerFacet::from(facet)),
            70 => Self::Ursa(UrsaFacet::from(facet)),
            71 => Self::SpiritBreaker(SpiritBreakerFacet::from(facet)),
            72 => Self::Gyrocopter(GyrocopterFacet::from(facet)),
            73 => Self::Alchemist(AlchemistFacet::from(facet)),
            74 => Self::Invoker(InvokerFacet::from(facet)),
            75 => Self::Silencer(SilencerFacet::from(facet)),
            76 => Self::ObsidianDestroyer(ObsidianDestroyerFacet::from(facet)),
            77 => Self::Lycan(LycanFacet::from(facet)),
            78 => Self::Brewmaster(BrewmasterFacet::from(facet)),
            79 => Self::ShadowDemon(ShadowDemonFacet::from(facet)),
            80 => Self::LoneDruid(LoneDruidFacet::from(facet)),
            81 => Self::ChaosKnight(ChaosKnightFacet::from(facet)),
            82 => Self::Meepo(MeepoFacet::from(facet)),
            83 => Self::Treant(TreantFacet::from(facet)),
            84 => Self::OgreMagi(OgreMagiFacet::from(facet)),
            85 => Self::Undying(UndyingFacet::from(facet)),
            86 => Self::Rubick(RubickFacet::from(facet)),
            87 => Self::Disruptor(DisruptorFacet::from(facet)),
            88 => Self::NyxAssassin(NyxAssassinFacet::from(facet)),
            89 => Self::NagaSiren(NagaSirenFacet::from(facet)),
            90 => Self::KeeperOfTheLight(KeeperOfTheLightFacet::from(facet)),
            91 => Self::Wisp(WispFacet::from(facet)),
            92 => Self::Visage(VisageFacet::from(facet)),
            93 => Self::Slark(SlarkFacet::from(facet)),
            94 => Self::Medusa(MedusaFacet::from(facet)),
            95 => Self::TrollWarlord(TrollWarlordFacet::from(facet)),
            96 => Self::Centaur(CentaurFacet::from(facet)),
            97 => Self::Magnataur(MagnataurFacet::from(facet)),
            98 => Self::Shredder(ShredderFacet::from(facet)),
            99 => Self::Bristleback(BristlebackFacet::from(facet)),
            100 => Self::Tusk(TuskFacet::from(facet)),
            101 => Self::SkywrathMage(SkywrathMageFacet::from(facet)),
            102 => Self::Abaddon(AbaddonFacet::from(facet)),
            103 => Self::ElderTitan(ElderTitanFacet::from(facet)),
            104 => Self::LegionCommander(LegionCommanderFacet::from(facet)),
            105 => Self::Techies(TechiesFacet::from(facet)),
            106 => Self::EmberSpirit(EmberSpiritFacet::from(facet)),
            107 => Self::EarthSpirit(EarthSpiritFacet::from(facet)),
            108 => Self::AbyssalUnderlord(AbyssalUnderlordFacet::from(facet)),
            109 => Self::Terrorblade(TerrorbladeFacet::from(facet)),
            110 => Self::Phoenix(PhoenixFacet::from(facet)),
            111 => Self::Oracle(OracleFacet::from(facet)),
            112 => Self::WinterWyvern(WinterWyvernFacet::from(facet)),
            113 => Self::ArcWarden(ArcWardenFacet::from(facet)),
            114 => Self::MonkeyKing(MonkeyKingFacet::from(facet)),
            119 => Self::DarkWillow(DarkWillowFacet::from(facet)),
            120 => Self::Pangolier(PangolierFacet::from(facet)),
            121 => Self::Grimstroke(GrimstrokeFacet::from(facet)),
            123 => Self::Hoodwink(HoodwinkFacet::from(facet)),
            126 => Self::VoidSpirit(VoidSpiritFacet::from(facet)),
            128 => Self::Snapfire(SnapfireFacet::from(facet)),
            129 => Self::Mars(MarsFacet::from(facet)),
            131 => Self::Ringmaster(RingmasterFacet::from(facet)),
            135 => Self::Dawnbreaker(DawnbreakerFacet::from(facet)),
            136 => Self::Marci(MarciFacet::from(facet)),
            137 => Self::PrimalBeast(PrimalBeastFacet::from(facet)),
            138 => Self::Muerta(MuertaFacet::from(facet)),
            145 => Self::Kez(KezFacet::from(facet)),
            id => Self::Unknown((id, facet)),
        }
    }
}

impl From<Hero> for (u8, u8) {
    fn from(hero: Hero) -> Self {
        match hero {
            Hero::Antimage(facet) => (1, u8::from(facet)),
            Hero::Axe(facet) => (2, u8::from(facet)),
            Hero::Bane(facet) => (3, u8::from(facet)),
            Hero::Bloodseeker(facet) => (4, u8::from(facet)),
            Hero::CrystalMaiden(facet) => (5, u8::from(facet)),
            Hero::DrowRanger(facet) => (6, u8::from(facet)),
            Hero::Earthshaker(facet) => (7, u8::from(facet)),
            Hero::Juggernaut(facet) => (8, u8::from(facet)),
            Hero::Mirana(facet) => (9, u8::from(facet)),
            Hero::Morphling(facet) => (10, u8::from(facet)),
            Hero::Nevermore(facet) => (11, u8::from(facet)),
            Hero::PhantomLancer(facet) => (12, u8::from(facet)),
            Hero::Puck(facet) => (13, u8::from(facet)),
            Hero::Pudge(facet) => (14, u8::from(facet)),
            Hero::Razor(facet) => (15, u8::from(facet)),
            Hero::SandKing(facet) => (16, u8::from(facet)),
            Hero::StormSpirit(facet) => (17, u8::from(facet)),
            Hero::Sven(facet) => (18, u8::from(facet)),
            Hero::Tiny(facet) => (19, u8::from(facet)),
            Hero::Vengefulspirit(facet) => (20, u8::from(facet)),
            Hero::Windrunner(facet) => (21, u8::from(facet)),
            Hero::Zuus(facet) => (22, u8::from(facet)),
            Hero::Kunkka(facet) => (23, u8::from(facet)),
            Hero::Lina(facet) => (25, u8::from(facet)),
            Hero::Lion(facet) => (26, u8::from(facet)),
            Hero::ShadowShaman(facet) => (27, u8::from(facet)),
            Hero::Slardar(facet) => (28, u8::from(facet)),
            Hero::Tidehunter(facet) => (29, u8::from(facet)),
            Hero::WitchDoctor(facet) => (30, u8::from(facet)),
            Hero::Lich(facet) => (31, u8::from(facet)),
            Hero::Riki(facet) => (32, u8::from(facet)),
            Hero::Enigma(facet) => (33, u8::from(facet)),
            Hero::Tinker(facet) => (34, u8::from(facet)),
            Hero::Sniper(facet) => (35, u8::from(facet)),
            Hero::Necrolyte(facet) => (36, u8::from(facet)),
            Hero::Warlock(facet) => (37, u8::from(facet)),
            Hero::Beastmaster(facet) => (38, u8::from(facet)),
            Hero::Queenofpain(facet) => (39, u8::from(facet)),
            Hero::Venomancer(facet) => (40, u8::from(facet)),
            Hero::FacelessVoid(facet) => (41, u8::from(facet)),
            Hero::SkeletonKing(facet) => (42, u8::from(facet)),
            Hero::DeathProphet(facet) => (43, u8::from(facet)),
            Hero::PhantomAssassin(facet) => (44, u8::from(facet)),
            Hero::Pugna(facet) => (45, u8::from(facet)),
            Hero::TemplarAssassin(facet) => (46, u8::from(facet)),
            Hero::Viper(facet) => (47, u8::from(facet)),
            Hero::Luna(facet) => (48, u8::from(facet)),
            Hero::DragonKnight(facet) => (49, u8::from(facet)),
            Hero::Dazzle(facet) => (50, u8::from(facet)),
            Hero::Rattletrap(facet) => (51, u8::from(facet)),
            Hero::Leshrac(facet) => (52, u8::from(facet)),
            Hero::Furion(facet) => (53, u8::from(facet)),
            Hero::LifeStealer(facet) => (54, u8::from(facet)),
            Hero::DarkSeer(facet) => (55, u8::from(facet)),
            Hero::Clinkz(facet) => (56, u8::from(facet)),
            Hero::Omniknight(facet) => (57, u8::from(facet)),
            Hero::Enchantress(facet) => (58, u8::from(facet)),
            Hero::Huskar(facet) => (59, u8::from(facet)),
            Hero::NightStalker(facet) => (60, u8::from(facet)),
            Hero::Broodmother(facet) => (61, u8::from(facet)),
            Hero::BountyHunter(facet) => (62, u8::from(facet)),
            Hero::Weaver(facet) => (63, u8::from(facet)),
            Hero::Jakiro(facet) => (64, u8::from(facet)),
            Hero::Batrider(facet) => (65, u8::from(facet)),
            Hero::Chen(facet) => (66, u8::from(facet)),
            Hero::Spectre(facet) => (67, u8::from(facet)),
            Hero::AncientApparition(facet) => (68, u8::from(facet)),
            Hero::DoomBringer(facet) => (69, u8::from(facet)),
            Hero::Ursa(facet) => (70, u8::from(facet)),
            Hero::SpiritBreaker(facet) => (71, u8::from(facet)),
            Hero::Gyrocopter(facet) => (72, u8::from(facet)),
            Hero::Alchemist(facet) => (73, u8::from(facet)),
            Hero::Invoker(facet) => (74, u8::from(facet)),
            Hero::Silencer(facet) => (75, u8::from(facet)),
            Hero::ObsidianDestroyer(facet) => (76, u8::from(facet)),
            Hero::Lycan(facet) => (77, u8::from(facet)),
            Hero::Brewmaster(facet) => (78, u8::from(facet)),
            Hero::ShadowDemon(facet) => (79, u8::from(facet)),
            Hero::LoneDruid(facet) => (80, u8::from(facet)),
            Hero::ChaosKnight(facet) => (81, u8::from(facet)),
            Hero::Meepo(facet) => (82, u8::from(facet)),
            Hero::Treant(facet) => (83, u8::from(facet)),
            Hero::OgreMagi(facet) => (84, u8::from(facet)),
            Hero::Undying(facet) => (85, u8::from(facet)),
            Hero::Rubick(facet) => (86, u8::from(facet)),
            Hero::Disruptor(facet) => (87, u8::from(facet)),
            Hero::NyxAssassin(facet) => (88, u8::from(facet)),
            Hero::NagaSiren(facet) => (89, u8::from(facet)),
            Hero::KeeperOfTheLight(facet) => (90, u8::from(facet)),
            Hero::Wisp(facet) => (91, u8::from(facet)),
            Hero::Visage(facet) => (92, u8::from(facet)),
            Hero::Slark(facet) => (93, u8::from(facet)),
            Hero::Medusa(facet) => (94, u8::from(facet)),
            Hero::TrollWarlord(facet) => (95, u8::from(facet)),
            Hero::Centaur(facet) => (96, u8::from(facet)),
            Hero::Magnataur(facet) => (97, u8::from(facet)),
            Hero::Shredder(facet) => (98, u8::from(facet)),
            Hero::Bristleback(facet) => (99, u8::from(facet)),
            Hero::Tusk(facet) => (100, u8::from(facet)),
            Hero::SkywrathMage(facet) => (101, u8::from(facet)),
            Hero::Abaddon(facet) => (102, u8::from(facet)),
            Hero::ElderTitan(facet) => (103, u8::from(facet)),
            Hero::LegionCommander(facet) => (104, u8::from(facet)),
            Hero::Techies(facet) => (105, u8::from(facet)),
            Hero::EmberSpirit(facet) => (106, u8::from(facet)),
            Hero::EarthSpirit(facet) => (107, u8::from(facet)),
            Hero::AbyssalUnderlord(facet) => (108, u8::from(facet)),
            Hero::Terrorblade(facet) => (109, u8::from(facet)),
            Hero::Phoenix(facet) => (110, u8::from(facet)),
            Hero::Oracle(facet) => (111, u8::from(facet)),
            Hero::WinterWyvern(facet) => (112, u8::from(facet)),
            Hero::ArcWarden(facet) => (113, u8::from(facet)),
            Hero::MonkeyKing(facet) => (114, u8::from(facet)),
            Hero::DarkWillow(facet) => (119, u8::from(facet)),
            Hero::Pangolier(facet) => (120, u8::from(facet)),
            Hero::Grimstroke(facet) => (121, u8::from(facet)),
            Hero::Hoodwink(facet) => (123, u8::from(facet)),
            Hero::VoidSpirit(facet) => (126, u8::from(facet)),
            Hero::Snapfire(facet) => (128, u8::from(facet)),
            Hero::Mars(facet) => (129, u8::from(facet)),
            Hero::Ringmaster(facet) => (131, u8::from(facet)),
            Hero::Dawnbreaker(facet) => (135, u8::from(facet)),
            Hero::Marci(facet) => (136, u8::from(facet)),
            Hero::PrimalBeast(facet) => (137, u8::from(facet)),
            Hero::Muerta(facet) => (138, u8::from(facet)),
            Hero::Kez(facet) => (145, u8::from(facet)),
            Hero::Unknown((id, facet)) => (id, facet),
        }
    }
}
