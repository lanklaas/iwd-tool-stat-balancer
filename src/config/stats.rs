use std::ops::{Deref, DerefMut};
use tracing::debug;

use super::{requirements::Role, Config};

const CONFIG: Config = Config::default();
const LEVEL: usize = CONFIG.level_min;

#[derive(Debug, Default, Clone, Copy)]
pub struct Stat {
    pub stat: &'static str,
    pub desc: &'static str,
    pub relates_to: Option<StaticRelations>,
    pub max: Option<usize>,
    pub roles: Option<Role>,
    pub no_scale: bool,
    pub initial: Option<usize>,
    pub min: Option<f32>,
    pub disabled: bool,
    pub value: f32,
}

impl PartialEq for Stat {
    fn eq(&self, other: &Self) -> bool {
        self.stat == other.stat
    }
}

impl PartialEq<str> for &Stat {
    fn eq(&self, other: &str) -> bool {
        self.stat == other
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StaticRelations {
    One([&'static str; 1]),
    Two([&'static str; 2]),
    Four([&'static str; 4]),
    Five([&'static str; 5]),
    Nine([&'static str; 9]),
}

#[derive(Debug)]
pub struct Stats(pub Vec<Stat>);

impl Stats {
    pub fn get<R>(
        &self,
        stat_name: &str,
        level: Option<usize>,
        tier: Option<&str>,
        role: Option<R>,
    ) -> f32
    where
        std::option::Option<R>: std::cmp::PartialEq<std::option::Option<Role>>,
    {
        let node = &self.0;
        let found_stat = self
            .0
            .iter()
            .find(|x| x.stat == stat_name)
            .expect("Should be there");

        let mut res = found_stat.value;

        if !found_stat.no_scale && level.is_some() {
            res *= level.unwrap() as f32
                * node
                    .iter()
                    .find(|x| x == "statMultPerLevel")
                    .expect("statMultPerLevel to exist")
                    .value;
        }

        match (found_stat.roles, tier) {
            (Some(roles), Some(tier)) => {
                res *= CONFIG.pieces_of_gear as f32
                    * (CONFIG.useful_stats_per_gear_piece.get(tier)
                        / self.0.iter().filter(|s| role == s.roles).count() as f32)
                    * CONFIG.roll_perfection_of_stats.get(tier);
            }
            other => {
                debug!("Stats get other condition: {other:?}");
            }
        }

        if let Some(max) = found_stat.max {
            if res > max as f32 {
                res = max as f32;
            }
        }

        res
    }

    pub fn get_mob(&self, stat_name: &str, level: Option<usize>) -> f32 {
        let node = &self.0;
        let found_stat = self
            .0
            .iter()
            .find(|x| x.stat == stat_name)
            .expect("Should be there");

        let mut res = found_stat.value;

        if !found_stat.no_scale && level.is_some() {
            res *= level.unwrap() as f32
                * node
                    .iter()
                    .find(|x| x == "statMultPerLevel")
                    .expect("statMultPerLevel to exist")
                    .value;
        }

        res
    }

    pub fn get_unscaled(&self, stat_name: &str) -> f32 {
        self.get(stat_name, None, None, None)
    }
}

impl Deref for Stats {
    type Target = [Stat];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self(vec![
            Stat {
                stat: "avoidChance",
                desc: "Gives the player a chance to completely avoid damage",
                relates_to: Some(StaticRelations::Four([
                    "blockAttackChance",
                    "blockSpellChance",
                    "dodgeAttackChance",
                    "dodgeSpellChance",
                ])),
                max: Some(100),
                roles: Some(Role::Tank(1)),
                initial: Some(10),
                ..Default::default()
            },
            Stat {
                stat: "mainStat",
                desc: "Grants a character\"s main stat",
                relates_to: Some(StaticRelations::Four([
                    "str",
                    "int",
                    "dex",
                    "allAttributes",
                ])),
                roles: Some(Role::Both { dps: 1., tank: 1 }),
                initial: Some((LEVEL * 5) / CONFIG.pieces_of_gear),
                ..Default::default()
            },
            Stat {
                stat: "playerDmgBase",
                desc: "Base player damage dealt",
                relates_to: Some(StaticRelations::One(["playerDmgBase"])),
                no_scale: true,
                min: Some(0.1),
                ..Default::default()
            },
            Stat {
                stat: "mobDmgBase",
                desc: "Base mob damage dealt",
                relates_to: Some(StaticRelations::One(["mobDmgBase"])),
                no_scale: true,
                min: Some(0.1),
                ..Default::default()
            },
            Stat {
                stat: "bossDmgBase",
                desc: "Base boss damage dealt",
                relates_to: Some(StaticRelations::One(["bossDmgBase"])),
                no_scale: true,
                min: Some(0.1),
                ..Default::default()
            },
            Stat {
                stat: "playerDmgMult",
                desc: "Multiplier for player damage dealt",
                relates_to: Some(StaticRelations::One(["statMult"])),
                no_scale: true,
                ..Default::default()
            },
            Stat {
                stat: "mobDmgMult",
                desc: "Multiplier for regular mob damage dealt",
                relates_to: Some(StaticRelations::One(["statMult"])),
                min: Some(0.1),
                ..Default::default()
            },
            Stat {
                stat: "bossDmgMult",
                desc: "Multiplier for boss mob damage dealt",
                relates_to: Some(StaticRelations::One(["statMult"])),
                min: Some(0.1),
                ..Default::default()
            },
            Stat {
                stat: "mobHpMult",
                desc: "Multiplier for regular mob hp",
                relates_to: Some(StaticRelations::One(["statMult"])),
                min: Some(0.1),
                ..Default::default()
            },
            Stat {
                stat: "bossHpMult",
                desc: "Multiplier for boss hp",
                relates_to: Some(StaticRelations::One(["statMult"])),
                min: Some(0.1),
                ..Default::default()
            },
            Stat {
                stat: "globalDmgPercent",
                desc: "Blanket multiplier for all damage types",
                relates_to: Some(StaticRelations::Nine([
                    "dmgPercent",
                    "physicalPercent",
                    "spellPercent",
                    "physicalPercent",
                    "elementArcanePercent",
                    "elementFrostPercent",
                    "elementFirePercent",
                    "elementHolyPercent",
                    "elementPoisonPercent",
                ])),
                roles: Some(Role::Dps(1)),
                ..Default::default()
            },
            Stat {
                stat: "globalCritChance",
                desc: "Chance to deal critical damage for all forms of damage",
                relates_to: Some(StaticRelations::One(["addCritChance"])),
                max: Some(100),
                roles: Some(Role::Dps(1)),
                ..Default::default()
            },
            Stat {
                stat: "attackSpellCritChance",
                desc: "Chance to deal critical damage for all attacks or spells",
                relates_to: Some(StaticRelations::Two([
                    "addAttackCritChance",
                    "addSpellCritChance",
                ])),
                max: Some(100),
                roles: Some(Role::Dps(1)),
                ..Default::default()
            },
            Stat {
                stat: "globalCritMultiplier",
                desc: "Multiplier for all forms of critical damage",
                relates_to: Some(StaticRelations::One(["addCritMultiplier"])),
                roles: Some(Role::Dps(1)),
                ..Default::default()
            },
            Stat {
                stat: "attackSpellCritMultiplier",
                desc: "Multiplier for all critical attacks or spells",
                relates_to: Some(StaticRelations::Two([
                    "addAttackCritMultiplier",
                    "addSpellCritMultiplier",
                ])),
                roles: Some(Role::Dps(1)),
                ..Default::default()
            },
            Stat {
                stat: "baseCritChance",
                desc: "Base crit chance for characters",
                relates_to: Some(StaticRelations::One(["baseCritChance"])),
                initial: Some(1),
                max: Some(100),
                no_scale: true,
                ..Default::default()
            },
            Stat {
                stat: "baseCritMultiplier",
                desc: "Base crit multiplier for characters",
                relates_to: Some(StaticRelations::One(["baseCritMultiplier"])),
                initial: Some(150),
                min: Some(150.),
                no_scale: true,
                ..Default::default()
            },
            //Mitigate
            Stat {
                stat: "elementAllResist",
                desc: "Resistance against all elemental damage",
                relates_to: Some(StaticRelations::One(["elementAllResist"])),
                roles: Some(Role::Both { tank: 1, dps: 0.3 }),
                ..Default::default()
            },
            Stat {
                stat: "elementResist",
                desc: "Resistance against specific forms of elemental damage",
                relates_to: Some(StaticRelations::Five([
                    "elementArcaneResist",
                    "elementFrostResist",
                    "elementFireResist",
                    "elementHolyResist",
                    "elementPoisonResist",
                ])),
                roles: Some(Role::Both { tank: 1, dps: 0.3 }),
                ..Default::default()
            },
            Stat {
                stat: "armor",
                desc: "Mitigates physical damage",
                relates_to: Some(StaticRelations::One(["armor"])),
                roles: Some(Role::Both { tank: 1, dps: 0.5 }),
                initial: Some(LEVEL * 30),
                ..Default::default()
            },
            Stat {
                stat: "armorEffectMult",
                desc: "armorEffectMult",
                no_scale: true,
                ..Default::default()
            },
            //Life
            Stat {
                stat: "playerHpBase",
                desc: "Initial player hp",
                relates_to: Some(StaticRelations::One(["playerHpBase"])),
                no_scale: true,
                min: Some(10.),
                ..Default::default()
            },
            Stat {
                stat: "mobHpBase",
                desc: "Initial mob hp",
                relates_to: Some(StaticRelations::One(["mobHpBase"])),
                no_scale: true,
                min: Some(1.),
                ..Default::default()
            },
            Stat {
                stat: "bossHpBase",
                desc: "Initial boss hp",
                relates_to: Some(StaticRelations::One(["bossHpBase"])),
                no_scale: true,
                min: Some(10.),
                ..Default::default()
            },
            Stat {
                stat: "vit",
                desc: "Grants extra hp",
                relates_to: Some(StaticRelations::One(["vit"])),
                roles: Some(Role::Both { tank: 1, dps: 0.3 }),
                ..Default::default()
            },
            Stat {
                stat: "vitToHpMultiplier",
                desc: "Defines how much hp a player gets for each point of vit",
                relates_to: Some(StaticRelations::One(["statScales.vitToHp"])),
                ..Default::default()
            },
            Stat {
                stat: "regenHp",
                desc: "Regenerates HP per tick",
                relates_to: Some(StaticRelations::One(["regenHp"])),
                roles: Some(Role::Both { tank: 1, dps: 0.2 }),
                ..Default::default()
            },
            Stat {
                stat: "lifeOnHit",
                desc: "Gains life on physical damage dealt",
                relates_to: Some(StaticRelations::One(["lifeOnHit"])),
                roles: Some(Role::Both { tank: 1, dps: 0.3 }),
                disabled: true,
                ..Default::default()
            },
            //Mana
            Stat {
                stat: "manaMax",
                desc: "Grants extra mana",
                relates_to: Some(StaticRelations::One(["manaMax"])),
                disabled: true,
                ..Default::default()
            },
            Stat {
                stat: "regenMana",
                desc: "Grants mana per tick",
                relates_to: Some(StaticRelations::One(["regenMana"])),
                disabled: true,
                ..Default::default()
            },
            //Other
            Stat {
                stat: "attackSpellSpeed",
                desc: "Grants faster attack/cast times",
                relates_to: Some(StaticRelations::Two(["attackSpeed", "castSpeed"])),
                disabled: true,
                ..Default::default()
            },
            Stat {
                stat: "statMultPerLevel",
                desc: "Multiplier for all stats per level",
                no_scale: true,
                min: Some(0.1),
                ..Default::default()
            },
        ])
    }
}
