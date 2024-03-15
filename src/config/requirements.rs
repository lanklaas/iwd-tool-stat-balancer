#[derive(Debug, Clone, Copy)]
pub enum Role {
    Dps(usize),
    Tank(usize),
    Both { dps: f32, tank: usize },
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        matches!(self, other)
    }
}

#[derive(Debug)]
pub struct Tier1 {
    pub entry: Role,
    pub mid: Role,
}

#[derive(Debug)]
pub struct Tier2 {
    pub high: Role,
    pub op: Role,
}

#[derive(Debug)]
pub struct Requirements {
    pub seconds_to_kill_mob: Tier1,
    pub seconds_to_kill_boss: Tier2,
    pub seconds_to_be_killed_by_mob: Tier1,
    pub seconds_to_be_killed_by_boss: Tier2,
}

impl Requirements {
    pub const fn default() -> Self {
        Self {
            seconds_to_kill_mob: Tier1 {
                entry: Role::Both { dps: 5.0, tank: 10 },
                mid: Role::Both { dps: 3.0, tank: 8 },
            },
            seconds_to_kill_boss: Tier2 {
                high: Role::Both {
                    dps: 180.0,
                    tank: 900,
                },
                op: Role::Both {
                    dps: 45.0,
                    tank: 180,
                },
            },
            seconds_to_be_killed_by_mob: Tier1 {
                entry: Role::Both {
                    dps: 10.0,
                    tank: 20,
                },
                mid: Role::Both {
                    dps: 12.0,
                    tank: 40,
                },
            },
            seconds_to_be_killed_by_boss: Tier2 {
                high: Role::Both { dps: 7.0, tank: 40 },
                op: Role::Both { dps: 9.0, tank: 80 },
            },
        }
    }
}
