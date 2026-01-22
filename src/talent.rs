use crate::singleton;
use i18n_utils::I18n;
use strum_macros::EnumIter;

singleton!(Talents = Talents::new());

#[derive(Debug)]
pub struct Talents {
    toggled: Vec<Talent>,
}

impl Talents {
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

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd, EnumIter, I18n)]
pub enum Talent {
    // ----- Default -----
    #[i18n(sc = "激励")]
    Motivation,
    #[i18n(sc = "登山伙伴")]
    HikingBuddy,
    #[i18n(sc = "跑腿糯米加油站")]
    RunnerBackstop,
    #[i18n(sc = "额外住房")]
    ExtraHousing,
    #[i18n(sc = "浴火奋战")]
    BatheInFire,
    // ----- 2 Talent Points -----
    #[i18n(sc = "糯米多力量大")]
    StrengthInNumbers,
    #[i18n(sc = "爆米花")]
    Popcorn,
    #[i18n(sc = "子弹吐息")]
    BulletBreath,
    #[i18n(sc = "燥热")]
    Superhot,
    #[i18n(sc = "时间位移")]
    TimeshiftDelivery,
    #[i18n(sc = "爆破秀")]
    ExplosiveExposition,
    // ----- 5 Talent Point -----
    #[i18n(sc = "启发")]
    Inspiration,
    #[i18n(sc = "加特林热气球")]
    GatlingBalloon,
    #[i18n(sc = "顺其自然")]
    TheFlow,
    #[i18n(sc = "更大石堆")]
    BiggerPile,
    #[i18n(sc = "石堆越高")]
    TheHigherThePile,
    #[i18n(sc = "领航笔记")]
    Notes,
    // ----- 8 Talent Point -----
    #[i18n(sc = "转生权宜")]
    PrestigeExpedience,
    #[i18n(sc = "回到过去")]
    FutureToTheBack,
    #[i18n(sc = "群星计划")]
    ProjectConstellation,
    #[i18n(sc = "超级霜冻")]
    Supershatter,
    #[i18n(sc = "物归原主")]
    ReturnToSender,
    #[i18n(sc = "王之降临")]
    DescentOfTheKing,
    // ----- 11 Talent Point -----
    #[i18n(sc = "灵能元素大师")]
    ZybelliumMastery,
    #[i18n(sc = "导弹加特林糯米")]
    RocketGatler,
    #[i18n(sc = "冰与火之歌")]
    Frostfire,
    #[i18n(sc = "无限回响")]
    InfiniteReverberation,
    #[i18n(sc = "打折出售")]
    Deals,
    #[i18n(sc = "减少回收")]
    ReclamationMitigation,
    // ----- 14 Talent Point -----
    #[i18n(sc = "新建筑")]
    NewBuilding,
    #[i18n(sc = "强力连招")]
    ComboPower,
    #[i18n(sc = "弓箭的最后波纹")]
    ArrowsLastHurrah,
    #[allow(non_camel_case_types)]
    #[i18n(sc = "给我吐出来")]
    DONT_EAT_THAT,
    #[allow(non_camel_case_types)]
    #[i18n(sc = "无糯机.目标 = 远 || 无糯机.目标 = 近")]
    DRONE_TARGET_FAR_OR_DRONE_TARGET_CLOSE,
    #[i18n(sc = "易伤增强")]
    VulnerabilityEmpowerment,
    // ----- 17 Talent Point -----
    #[i18n(sc = "伤害倍率增加")]
    MultiplierGrowth,
    #[i18n(sc = "口袋导弹行动时间位移保留")]
    OperationPocketRocketTimeshiftRetainment,
    #[i18n(sc = "焦躁的灵能元素")]
    AgitatedZybe,
    #[i18n(sc = "糯米圣剑：释放神力")]
    GnorpcaliburUnleashed,
    #[i18n(sc = "现代战争")]
    ModernWarfare,
    #[i18n(sc = "无糯机容量")]
    DroneCapacity,
}
