pub mod mutation;
pub mod requirements;
pub mod stats;
pub struct TierScale {
    pub entry: f32,
    pub mid: f32,
    pub high: f32,
    pub op: f32,
}

impl TierScale {
    pub fn get(&self, name: &str) -> f32 {
        match name {
            "entry" => self.entry,
            "mid" => self.mid,
            "high" => self.high,
            "op" => self.op,
            other => panic!("{other} is not a valid tier scale"),
        }
    }
}

pub struct Config {
    pub level_min: usize,
    pub level_max: usize,
    pub player_mana_base: usize,
    pub ticks_taken_to_deal_dmg_base: usize,
    pub ticks_per_second: usize,
    pub stat_base: usize,
    pub pieces_of_gear: usize,
    pub tiers: [&'static str; 4],
    pub roles: [&'static str; 2],
    pub stats_per_gear_piece: TierScale,
    pub useful_stats_per_gear_piece: TierScale,
    pub roll_perfection_of_stats: TierScale,
}

impl Config {
    pub const fn default() -> Self {
        Self {
            level_min: 25,
            level_max: 25,
            player_mana_base: 10,
            ticks_taken_to_deal_dmg_base: 1,
            ticks_per_second: 1000 / 350,
            stat_base: 0,
            pieces_of_gear: 11,
            tiers: ["entry", "mid", "high", "op"],
            roles: ["dps", "tank"],
            stats_per_gear_piece: TierScale {
                entry: 1.,
                mid: 3.,
                high: 5.,
                op: 8.,
            },
            useful_stats_per_gear_piece: TierScale {
                entry: 0.5,
                mid: 1.5,
                high: 4.,
                op: 7.,
            },
            roll_perfection_of_stats: TierScale {
                entry: 0.5,
                mid: 0.6,
                high: 0.7,
                op: 0.8,
            },
        }
    }
}
