use super::{requirements::Role, Config};

const CONFIG: Config = Config::default();
const LEVEL: usize = CONFIG.level_min;

#[derive(Debug)]
pub struct Stat {
    stat: &'static str,
    desc: &'static str,
    relates_to: [Option<&'static str>; 10],
    max: Option<usize>,
    roles: Role,
    no_scale: bool,
    initial: usize,
    min: Option<f32>,
}

impl Stat {
    pub const fn default() -> [Self; 5] {
        [
            Self {
                stat: "avoidChance",
                desc: "Gives the player a chance to completely avoid damage",
                relates_to: [
                    "blockAttackChance".into(),
                    "blockSpellChance".into(),
                    "dodgeAttackChance".into(),
                    "dodgeSpellChance".into(),
                    None,
                ],
                max: Some(100),
                roles: Role::Tank(1),
                initial: 10,
            },
            Self {
                stat: "mainStat",
                desc: "Grants a character\"s main stat",
                relates_to: [
                    "str".into(),
                    "int".into(),
                    "dex".into(),
                    "allAttributes".into(),
                ],
                roles: Role::Both { dps: 1, tank: 1 },
                initial: (LEVEL * 5) / CONFIG.pieces_of_gear,
            },
            Self {
                stat: "playerDmgBase",
                desc: "Base player damage dealt",
                relates_to: ["playerDmgBase".into(), ...=None],
                no_scale: true,
                min: 0.1.into(),
                max: None,
            },
        ]
    }
}
// module.exports = [

// 	{
// 		stat: "mobDmgBase",
// 		desc: "Base mob damage dealt",
// 		relatesTo: [
// 			"mobDmgBase"
// 		],
// 		noScale: true,
// 		min: 0.1
// 	},
// 	{
// 		stat: "bossDmgBase",
// 		desc: "Base boss damage dealt",
// 		relatesTo: [
// 			"bossDmgBase"
// 		],
// 		noScale: true,
// 		min: 0.1
// 	},
// 	{
// 		stat: "playerDmgMult",
// 		desc: "Multiplier for player damage dealt",
// 		relatesTo: [
// 			"statMult"
// 		],
// 		noScale: true
// 	},
// 	{
// 		stat: "mobDmgMult",
// 		desc: "Multiplier for regular mob damage dealt",
// 		relatesTo: [
// 			"statMult"
// 		],
// 		min: 0.1
// 	},
// 	{
// 		stat: "bossDmgMult",
// 		desc: "Multiplier for boss mob damage dealt",
// 		relatesTo: [
// 			"statMult"
// 		],
// 		min: 0.1
// 	},
// 	{
// 		stat: "mobHpMult",
// 		desc: "Multiplier for regular mob hp",
// 		relatesTo: [
// 			"statMult"
// 		],
// 		min: 0.1
// 	},
// 	{
// 		stat: "bossHpMult",
// 		desc: "Multiplier for boss hp",
// 		relatesTo: [
// 			"statMult"
// 		],
// 		min: 0.1
// 	},
// 	{
// 		stat: "globalDmgPercent",
// 		desc: "Blanket multiplier for all damage types",
// 		relatesTo: [
// 			"dmgPercent",
// 			"physicalPercent",
// 			"spellPercent",
// 			"physicalPercent",
// 			"elementArcanePercent",
// 			"elementFrostPercent",
// 			"elementFirePercent",
// 			"elementHolyPercent",
// 			"elementPoisonPercent"
// 		],
// 		roles: {
// 			dps: 1
// 		}
// 	},
// 	{
// 		stat: "globalCritChance",
// 		desc: "Chance to deal critical damage for all forms of damage",
// 		relatesTo: [
// 			"addCritChance"
// 		],
// 		max: 100,
// 		roles: {
// 			dps: 1
// 		}
// 	},
// 	{
// 		stat: "attackSpellCritChance",
// 		desc: "Chance to deal critical damage for all attacks or spells",
// 		relatesTo: [
// 			"addAttackCritChance",
// 			"addSpellCritChance"
// 		],
// 		max: 100,
// 		roles: {
// 			dps: 1
// 		}
// 	},
// 	{
// 		stat: "globalCritMultiplier",
// 		desc: "Multiplier for all forms of critical damage",
// 		relatesTo: [
// 			"addCritMultiplier"
// 		],
// 		roles: {
// 			dps: 1
// 		}
// 	},
// 	{
// 		stat: "attackSpellCritMultiplier",
// 		desc: "Multiplier for all critical attacks or spells",
// 		relatesTo: [
// 			"addAttackCritMultiplier",
// 			"addSpellCritMultiplier"
// 		],
// 		roles: {
// 			dps: 1
// 		}
// 	},
// 	{
// 		stat: "baseCritChance",
// 		desc: "Base crit chance for characters",
// 		relatesTo: [
// 			"baseCritChance"
// 		],
// 		initial: 1,
// 		max: 100,
// 		noScale: true
// 	},
// 	{
// 		stat: "baseCritMultiplier",
// 		desc: "Base crit multiplier for characters",
// 		relatesTo: [
// 			"baseCritMultiplier"
// 		],
// 		initial: 150,
// 		min: 150,
// 		noScale: true,
// 		isStatic: true
// 	},
// 	//Mitigate
// 	{
// 		stat: "elementAllResist",
// 		desc: "Resistance against all elemental damage",
// 		relatesTo: [
// 			"elementAllResist"
// 		],
// 		roles: {
// 			tank: 1,
// 			dps: 0.3
// 		}
// 	},
// 	{
// 		stat: "elementResist",
// 		desc: "Resistance against specific forms of elemental damage",
// 		relatesTo: [
// 			"elementArcaneResist",
// 			"elementFrostResist",
// 			"elementFireResist",
// 			"elementHolyResist",
// 			"elementPoisonResist"
// 		],
// 		roles: {
// 			tank: 1,
// 			dps: 0.3
// 		}
// 	},
// 	{
// 		stat: "armor",
// 		desc: "Mitigates physical damage",
// 		relatesTo: [
// 			"armor"
// 		],
// 		roles: {
// 			tank: 1,
// 			dps: 0.5
// 		},
// 		initial: (level * 30)
// 	},
// 	{
// 		stat: "armorEffectMult",
// 		noScale: true
// 	},
// 	//Life
// 	{
// 		stat: "playerHpBase",
// 		desc: "Initial player hp",
// 		relatesTo: [
// 			"playerHpBase"
// 		],
// 		noScale: true,
// 		min: 10
// 	},
// 	{
// 		stat: "mobHpBase",
// 		desc: "Initial mob hp",
// 		relatesTo: [
// 			"mobHpBase"
// 		],
// 		noScale: true,
// 		min: 1
// 	},
// 	{
// 		stat: "bossHpBase",
// 		desc: "Initial boss hp",
// 		relatesTo: [
// 			"bossHpBase"
// 		],
// 		noScale: true,
// 		min: 10
// 	},
// 	{
// 		stat: "vit",
// 		desc: "Grants extra hp",
// 		relatesTo: [
// 			"vit"
// 		],
// 		roles: {
// 			tank: 1,
// 			dps: 0.3
// 		}
// 	},
// 	{
// 		stat: "vitToHpMultiplier",
// 		desc: "Defines how much hp a player gets for each point of vit",
// 		relatesTo: [
// 			"statScales.vitToHp"
// 		]
// 	},
// 	{
// 		stat: "regenHp",
// 		desc: "Regenerates HP per tick",
// 		relatesTo: [
// 			"regenHp"
// 		],
// 		roles: {
// 			tank: 1,
// 			dps: 0.2
// 		}
// 	},
// 	{
// 		stat: "lifeOnHit",
// 		desc: "Gains life on physical damage dealt",
// 		relatesTo: [
// 			"lifeOnHit"
// 		],
// 		roles: {
// 			tank: 1,
// 			dps: 0.3
// 		},
// 		disabled: true
// 	},
// 	//Mana
// 	{
// 		stat: "manaMax",
// 		desc: "Grants extra mana",
// 		relatesTo: [
// 			"manaMax"
// 		],
// 		disabled: true
// 	},
// 	{
// 		stat: "regenMana",
// 		desc: "Grants mana per tick",
// 		relatesTo: [
// 			"regenMana"
// 		],
// 		disabled: true
// 	},
// 	//Other
// 	{
// 		stat: "attackSpellSpeed",
// 		desc: "Grants faster attack/cast times",
// 		relatesTo: [
// 			"attackSpeed",
// 			"castSpeed"
// 		],
// 		disabled: true
// 	},
// 	{
// 		stat: "statMultPerLevel",
// 		desc: "Multiplier for all stats per level",
// 		noScale: true,
// 		min: 0.1
// 	}
// ];
