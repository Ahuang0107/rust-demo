static mut TALENTS: Talents = Talents::new();

#[derive(Debug)]
pub struct Talents {
    toggled: Vec<Talent>,
}

impl Talents {
    #[inline]
    pub const fn single() -> &'static Talents {
        #[allow(static_mut_refs)]
        unsafe {
            &TALENTS
        }
    }
    #[inline]
    pub const fn single_mut() -> &'static mut Talents {
        #[allow(static_mut_refs)]
        unsafe {
            &mut TALENTS
        }
    }
    #[inline]
    pub const fn new() -> Talents {
        Self { toggled: vec![] }
    }
    #[inline]
    pub fn if_toggle(&self, key: Talent) -> bool {
        self.toggled.contains(&key)
    }
    #[inline]
    pub fn toggle(&mut self, key: Talent) {
        if !self.toggled.contains(&key) {
            self.toggled.push(key);
            self.toggled.sort();
        }
    }
    #[inline]
    pub fn untoggle(&mut self, key: Talent) {
        self.toggled.retain(|&x| x != key);
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub enum Talent {
    // ----- Default -----
    Motivation,
    HikingBuddy,
    RunnerBackstop,
    ExtraHousing,
    BatheInFire,
    // ----- 2 Talent Points -----
    StrengthInNumbers,
    Popcorn,
    BulletBreath,
    Superhot,
    TimeshiftDelivery,
    ExplosiveExposition,
    // ----- 5 Talent Point -----
    Inspiration,
    GatlingBalloon,
    TheFlow,
    BiggerPile,
    TheHigherThePile,
    Notes,
    // ----- 8 Talent Point -----
    PrestigeExpedience,
    FutureToTheBack,
    ProjectConstellation,
    Supershatter,
    ReturnToSender,
    DescentOfTheKing,
    // ----- 11 Talent Point -----
    ZybelliumMastery,
    RocketGatler,
    Frostfire,
    InfiniteReverberation,
    Deals,
    ReclamationMitigation,
    // ----- 14 Talent Point -----
    NewBuilding,
    ComboPower,
    ArrowsLastHurrah,
    #[allow(non_camel_case_types)]
    DONT_EAT_THAT,
    #[allow(non_camel_case_types)]
    DRONE_TARGET_FAR_OR_DRONE_TARGET_CLOSE,
    VulnerabilityEmpowerment,
    // ----- 17 Talent Point -----
    MultiplierGrowth,
    OperationPocketRocketTimeshiftRetainment,
    AgitatedZybe,
    GnorpcaliburUnleashed,
    ModernWarfare,
    DroneCapacity,
}
