use crate::dota2::define_dota2_enum;

define_dota2_enum! {
    /// dota2 Item enums.
    pub enum Item: u16 {
        AbilityBase = 0,
        Blink = 1,
        BladesOfAttack = 2,
        Broadsword = 3,
        Chainmail = 4,
        Claymore = 5,
        HelmOfIronWill = 6,
        Javelin = 7,
        MithrilHammer = 8,
        Platemail = 9,
        Quarterstaff = 10,
        QuellingBlade = 11,
        RingOfProtection = 12,
        Gauntlets = 13,
        Slippers = 14,
        Mantle = 15,
        Branches = 16,
        BeltOfStrength = 17,
        BootsOfElves = 18,
        Robe = 19,
        Circlet = 20,
        OgreAxe = 21,
        BladeOfAlacrity = 22,
        StaffOfWizardry = 23,
        UltimateOrb = 24,
        Gloves = 25,
        Lifesteal = 26,
        RingOfRegen = 27,
        SobiMask = 28,
        Boots = 29,
        Gem = 30,
        Cloak = 31,
        TalismanOfEvasion = 32,
        Cheese = 33,
        MagicStick = 34,
        RecipeMagicWand = 35,
        MagicWand = 36,
        Ghost = 37,
        Clarity = 38,
        Flask = 39,
        Dust = 40,
        Bottle = 41,
        WardObserver = 42,
        WardSentry = 43,
        Tango = 44,
        Courier = 45,
        Tpscroll = 46,
        RecipeTravelBoots = 47,
        TravelBoots = 48,
        RecipePhaseBoots = 49,
        PhaseBoots = 50,
        DemonEdge = 51,
        Eagle = 52,
        Reaver = 53,
        Relic = 54,
        Hyperstone = 55,
        RingOfHealth = 56,
        VoidStone = 57,
        MysticStaff = 58,
        EnergyBooster = 59,
        PointBooster = 60,
        VitalityBooster = 61,
        RecipePowerTreads = 62,
        PowerTreads = 63,
        RecipeHandOfMidas = 64,
        HandOfMidas = 65,
        RecipeOblivionStaff = 66,
        OblivionStaff = 67,
        RecipePers = 68,
        Pers = 69,
        RecipePoorMansShield = 70,
        PoorMansShield = 71,
        RecipeBracer = 72,
        Bracer = 73,
        RecipeWraithBand = 74,
        WraithBand = 75,
        RecipeNullTalisman = 76,
        NullTalisman = 77,
        RecipeMekansm = 78,
        Mekansm = 79,
        RecipeVladmir = 80,
        Vladmir = 81,
        RecipeBuckler = 85,
        Buckler = 86,
        RecipeRingOfBasilius = 87,
        RingOfBasilius = 88,
        RecipePipe = 89,
        Pipe = 90,
        RecipeUrnOfShadows = 91,
        UrnOfShadows = 92,
        RecipeHeaddress = 93,
        Headdress = 94,
        RecipeSheepstick = 95,
        Sheepstick = 96,
        RecipeOrchid = 97,
        Orchid = 98,
        RecipeCyclone = 99,
        Cyclone = 100,
        RecipeForceStaff = 101,
        ForceStaff = 102,
        RecipeDagon = 103,
        Dagon = 104,
        RecipeNecronomicon = 105,
        Necronomicon = 106,
        RecipeUltimateScepter = 107,
        UltimateScepter = 108,
        RecipeRefresher = 109,
        Refresher = 110,
        RecipeAssault = 111,
        Assault = 112,
        RecipeHeart = 113,
        Heart = 114,
        RecipeBlackKingBar = 115,
        BlackKingBar = 116,
        Aegis = 117,
        RecipeShivasGuard = 118,
        ShivasGuard = 119,
        RecipeBloodstone = 120,
        Bloodstone = 121,
        RecipeSphere = 122,
        Sphere = 123,
        RecipeVanguard = 124,
        Vanguard = 125,
        RecipeBladeMail = 126,
        BladeMail = 127,
        RecipeSoulBooster = 128,
        SoulBooster = 129,
        RecipeHoodOfDefiance = 130,
        HoodOfDefiance = 131,
        RecipeRapier = 132,
        Rapier = 133,
        RecipeMonkeyKingBar = 134,
        MonkeyKingBar = 135,
        RecipeRadiance = 136,
        Radiance = 137,
        RecipeButterfly = 138,
        Butterfly = 139,
        RecipeGreaterCrit = 140,
        GreaterCrit = 141,
        RecipeBasher = 142,
        Basher = 143,
        RecipeBfury = 144,
        Bfury = 145,
        RecipeManta = 146,
        Manta = 147,
        RecipeLesserCrit = 148,
        LesserCrit = 149,
        RecipeArmlet = 150,
        Armlet = 151,
        InvisSword = 152,
        RecipeSangeAndYasha = 153,
        SangeAndYasha = 154,
        RecipeSatanic = 155,
        Satanic = 156,
        RecipeMjollnir = 157,
        Mjollnir = 158,
        RecipeSkadi = 159,
        Skadi = 160,
        RecipeSange = 161,
        Sange = 162,
        RecipeHelmOfTheDominator = 163,
        HelmOfTheDominator = 164,
        RecipeMaelstrom = 165,
        Maelstrom = 166,
        RecipeDesolator = 167,
        Desolator = 168,
        RecipeYasha = 169,
        Yasha = 170,
        RecipeMaskOfMadness = 171,
        MaskOfMadness = 172,
        RecipeDiffusalBlade = 173,
        DiffusalBlade = 174,
        RecipeEtherealBlade = 175,
        EtherealBlade = 176,
        RecipeSoulRing = 177,
        SoulRing = 178,
        RecipeArcaneBoots = 179,
        ArcaneBoots = 180,
        OrbOfVenom = 181,
        StoutShield = 182,
        RecipeInvisSword = 183,
        RecipeAncientJanggo = 184,
        AncientJanggo = 185,
        RecipeMedallionOfCourage = 186,
        MedallionOfCourage = 187,
        SmokeOfDeceit = 188,
        RecipeVeilOfDiscord = 189,
        VeilOfDiscord = 190,
        RecipeNecronomicon2 = 191,
        RecipeNecronomicon3 = 192,
        Necronomicon2 = 193,
        Necronomicon3 = 194,
        DiffusalBlade2 = 196,
        RecipeDagon2 = 197,
        RecipeDagon3 = 198,
        RecipeDagon4 = 199,
        RecipeDagon5 = 200,
        Dagon2 = 201,
        Dagon3 = 202,
        Dagon4 = 203,
        Dagon5 = 204,
        RecipeRodOfAtos = 205,
        RodOfAtos = 206,
        RecipeAbyssalBlade = 207,
        AbyssalBlade = 208,
        RecipeHeavensHalberd = 209,
        HeavensHalberd = 210,
        RecipeRingOfAquila = 211,
        RingOfAquila = 212,
        RecipeTranquilBoots = 213,
        TranquilBoots = 214,
        ShadowAmulet = 215,
        EnchantedMango = 216,
        RecipeWardDispenser = 217,
        WardDispenser = 218,
        RecipeTravelBoots2 = 219,
        TravelBoots2 = 220,
        RecipeLotusOrb = 221,
        RecipeMeteorHammer = 222,
        MeteorHammer = 223,
        RecipeNullifier = 224,
        Nullifier = 225,
        LotusOrb = 226,
        RecipeSolarCrest = 227,
        RecipeOctarineCore = 228,
        SolarCrest = 229,
        RecipeGuardianGreaves = 230,
        GuardianGreaves = 231,
        AetherLens = 232,
        RecipeAetherLens = 233,
        RecipeDragonLance = 234,
        OctarineCore = 235,
        DragonLance = 236,
        FaerieFire = 237,
        RecipeIronTalon = 238,
        IronTalon = 239,
        BlightStone = 240,
        TangoSingle = 241,
        CrimsonGuard = 242,
        RecipeCrimsonGuard = 243,
        WindLace = 244,
        RecipeBloodthorn = 245,
        RecipeMoonShard = 246,
        MoonShard = 247,
        RecipeSilverEdge = 248,
        SilverEdge = 249,
        Bloodthorn = 250,
        RecipeEchoSabre = 251,
        EchoSabre = 252,
        RecipeGlimmerCape = 253,
        GlimmerCape = 254,
        RecipeAeonDisk = 255,
        AeonDisk = 256,
        TomeOfKnowledge = 257,
        RecipeKaya = 258,
        Kaya = 259,
        RefresherShard = 260,
        Crown = 261,
        RecipeHurricanePike = 262,
        HurricanePike = 263,
        InfusedRaindrop = 265,
        RecipeSpiritVessel = 266,
        SpiritVessel = 267,
        RecipeHolyLocket = 268,
        HolyLocket = 269,
        RecipeUltimateScepter2 = 270,
        UltimateScepter2 = 271,
        RecipeKayaAndSange = 272,
        KayaAndSange = 273,
        RecipeYashaAndKaya = 274,
        RecipeTrident = 275,
        ComboBreaker = 276,
        YashaAndKaya = 277,
        RingOfTarrasque = 279,
        FlyingCourier = 286,
        KeenOptic = 287,
        GroveBow = 288,
        QuickeningCharm = 289,
        PhilosophersStone = 290,
        ForceBoots = 291,
        Desolator2 = 292,
        PhoenixAsh = 293,
        SeerStone = 294,
        GreaterMango = 295,
        VampireFangs = 297,
        CraggyCoat = 298,
        GreaterFaerieFire = 299,
        TimelessRelic = 300,
        MirrorShield = 301,
        Elixer = 302,
        RecipeIronwoodTree = 303,
        IronwoodTree = 304,
        RoyalJelly = 305,
        PupilsGift = 306,
        TomeOfAghanim = 307,
        RepairKit = 308,
        MindBreaker = 309,
        ThirdEye = 310,
        SpellPrism = 311,
        Horizon = 312,
        FusionRune = 313,
        RecipeFallenSky = 317,
        PrincesKnife = 325,
        SpiderLegs = 326,
        HelmOfTheUndying = 327,
        MangoTree = 328,
        WitlessShako = 330,
        Vambrace = 331,
        ImpClaw = 334,
        Flicker = 335,
        SpyGadget = 336,
        ArcaneRing = 349,
        OceanHeart = 354,
        BroomHandle = 355,
        TrustyShovel = 356,
        NetherShawl = 357,
        DragonScale = 358,
        EssenceRing = 359,
        ClumsyNet = 360,
        EnchantedQuiver = 361,
        NinjaGear = 362,
        IllusionstsCape = 363,
        HavocHammer = 364,
        PanicButton = 365,
        Apex = 366,
        Ballista = 367,
        WoodlandStriders = 368,
        Trident = 369,
        Demonicon = 370,
        FallenSky = 371,
        PirateHat = 372,
        DimensionalDoorway = 373,
        ExMachina = 374,
        FadedBroach = 375,
        PaladinSword = 376,
        MinotaurHorn = 377,
        OrbOfDestruction = 378,
        TheLeveller = 379,
        TitanSliver = 381,
        VoodooMask = 473,
        BlitzKnuckles = 485,
        RecipeWitchBlade = 533,
        WitchBlade = 534,
        ChippedVest = 565,
        WizardGlass = 566,
        OrbOfCorrosion = 569,
        GlovesOfTravel = 570,
        TricksterCloak = 571,
        ElvenTunic = 573,
        CloakOfFlames = 574,
        VenomGland = 575,
        GladiatorHelm = 576,
        PossessedMask = 577,
        AncientPerseverance = 578,
        Oakheart = 582,
        Stormcrafter = 585,
        OverflowingElixir = 588,
        MysteriousHat = 589,
        FluffyHat = 593,
        FalconBlade = 596,
        RecipeMageSlayer = 597,
        MageSlayer = 598,
        RecipeFalconBlade = 599,
        OverwhelmingBlink = 600,
        SwiftBlink = 603,
        ArcaneBlink = 604,
        RecipeArcaneBlink = 606,
        RecipeSwiftBlink = 607,
        RecipeOverwhelmingBlink = 608,
        AghanimsShard = 609,
        WindWaker = 610,
        RecipeWindWaker = 612,
        RecipeHelmOfTheOverlord = 633,
        HelmOfTheOverlord = 635,
        StarMace = 637,
        PentaEdgedSword = 638,
        RecipeOrbOfCorrosion = 640,
        RecipeGrandmastersGlaive = 653,
        GrandmastersGlaive = 655,
        Warhammer = 674,
        PsychicHeadband = 675,
        CeremonialRobe = 676,
        BookOfShadows = 677,
        GiantsRing = 678,
        VengeancesShadow = 679,
        Bullwhip = 680,
        QuicksilverAmulet = 686,
        RecipeEternalShroud = 691,
        EternalShroud = 692,
        AghanimsShardRoshan = 725,
        UltimateScepterRoshan = 727,
        Satchel = 731,
        AssassinsDagger = 824,
        AsceticCap = 825,
        SamplePicker = 826,
        IcarusWings = 827,
        Misericorde = 828,
        ForceField = 829,
        BlackPowderBag = 834,
        Paintball = 835,
        LightRobes = 836,
        HeavyBlade = 837,
        UnstableWand = 838,
        FortitudeRing = 839,
        PogoStick = 840,
        MechanicalArm = 849,
        RecipeWraithPact = 907,
        WraithPact = 908,
        RecipeRevenantsBrooch = 910,
        RevenantsBrooch = 911,
        RecipeBootsOfBearing = 930,
        BootsOfBearing = 931,
        SlimeVial = 938,
        Harpoon = 939,
        WandOfTheBrine = 940,
        SeedsOfSerenity = 945,
        LanceOfPursuit = 946,
        OccultBracelet = 947,
        TomeOfOmniscience = 948,
        OgreSealTotem = 949,
        DefiantShell = 950,
        ArcaneScout = 968,
        Barricade = 969,
        EyeOfTheVizier = 990,
        ManaclesOfPower = 998,
        BottomlessChalice = 1000,
        WandOfSanctitude = 1017,
        RiverPainter = 1021,
        RiverPainter2 = 1022,
        RiverPainter3 = 1023,
        RiverPainter4 = 1024,
        RiverPainter5 = 1025,
        RiverPainter6 = 1026,
        RiverPainter7 = 1027,
        MutationTombstone = 1028,
        SuperBlink = 1029,
        PocketTower = 1030,
        PocketRoshan = 1032,
        SpecialistsArray = 1076,
        DaggerOfRistul = 1077,
        MuertasGun = 1090,
        SamuraiTabi = 1091,
        RecipeHermesSandals = 1092,
        HermesSandals = 1093,
        RecipeLunarCrest = 1094,
        LunarCrest = 1095,
        RecipeDisperser = 1096,
        Disperser = 1097,
        RecipeSamuraiTabi = 1098,
        RecipeWitchesSwitch = 1099,
        WitchesSwitch = 1100,
        RecipeHarpoon = 1101,
        RecipePhylactery = 1106,
        Phylactery = 1107,
        Diadem = 1122,
        BloodGrenade = 1123,
        SparkOfCourage = 1124,
        Cornucopia = 1125,
        RecipePavise = 1127,
        Pavise = 1128,
        RoyaleWithCheese = 1154,
        AncientGuardian = 1156,
        SafetyBubble = 1157,
        WhisperOfTheDread = 1158,
        NemesisCurse = 1159,
        AvianasFeather = 1160,
        UnwaveringCondition = 1161,
        Halo = 1162,
        RecipeAetherialHalo = 1163,
        AetherialHalo = 1164,
        LightCollector = 1167,
        Rattlecage = 1168,
        BlackGrimoire = 1440,
        Grisgris = 1441,
        Gungir = 1466,
        CladdishSpyglass = 1487,
        RecipeGungir = 1565,
        RecipeCasterRapier = 1800,
        CasterRapier = 1801,
        TiaraOfSelemene = 1802,
        Doubloon = 1803,
        RoshansBanner = 1804,
        RecipeDevastator = 1805,
        Devastator = 1806,
        RecipeAngelsDemise = 1807,
        AngelsDemise = 1808,
        Tier1Token = 2091,
        Tier2Token = 2092,
        Tier3Token = 2093,
        Tier4Token = 2094,
        Tier5Token = 2095,
        VindicatorsAxe = 2096,
        DuelistGloves = 2097,
        HorizonsEquilibrium = 2098,
        BlightedSpirit = 2099,
        DandelionAmulet = 2190,
        TurtleShell = 2191,
        MartyrsPlate = 2192,
        GossamerCape = 2193,
        Famango = 4204,
        GreatFamango = 4205,
        GreaterFamango = 4206,
        RecipeGreatFamango = 4207,
        RecipeGreaterFamango = 4208,
        Ofrenda = 4300,
        OfrendaShovel = 4301,
        OfrendaPledge = 4302,
    }
}