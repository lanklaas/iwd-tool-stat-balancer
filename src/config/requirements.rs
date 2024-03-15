use std::ops::Deref;

#[derive(Debug, Clone, Copy, Default)]
pub enum Role {
    Dps(usize),
    Tank(usize),
    Both {
        dps: f32,
        tank: usize,
    },
    #[default]
    None,
}

impl Role {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Dps(_), Self::Dps(_))
                | (Self::Tank(_), Self::Tank(_))
                | (Self::Both { .. }, Self::Both { .. })
                | (Self::None, Self::None)
        )
    }
}

impl PartialEq<str> for Role {
    fn eq(&self, other: &str) -> bool {
        matches!(
            (other, self),
            ("dps", Self::Dps(_)) | ("tank", Self::Tank(_))
        )
    }
}

#[derive(Debug)]
pub struct Tier {
    pub name: &'static str,
    pub role: Role,
}

#[derive(Debug)]
pub struct Tiers([Tier; 4]);
impl Deref for Tiers {
    type Target = [Tier; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// pub high: Role,
//     pub mid: Role,
//     pub entry: Role,
//     pub op: Role,

#[derive(Debug)]
pub struct Requirement {
    pub name: &'static str,
    pub tiers: Tiers,
}

#[derive(Debug)]
pub struct Requirements([Requirement; 4]);
impl Deref for Requirements {
    type Target = [Requirement; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Requirements {
    pub const fn default() -> Self {
        Self([
            Requirement {
                name: "seconds_to_kill_mob",
                tiers: Tiers([
                    Tier {
                        name: "entry",
                        role: Role::Both { dps: 5.0, tank: 10 },
                    },
                    Tier {
                        name: "mid",
                        role: Role::Both { dps: 3.0, tank: 8 },
                    },
                    Tier {
                        name: "high",
                        role: Role::None,
                    },
                    Tier {
                        name: "op",
                        role: Role::None,
                    },
                ]),
            },
            Requirement {
                name: "seconds_to_kill_boss",
                tiers: Tiers([
                    Tier {
                        name: "high",
                        role: Role::Both {
                            dps: 180.0,
                            tank: 900,
                        },
                    },
                    Tier {
                        name: "op",
                        role: Role::Both {
                            dps: 45.0,
                            tank: 180,
                        },
                    },
                    Tier {
                        name: "mid",
                        role: Role::None,
                    },
                    Tier {
                        name: "entry",
                        role: Role::None,
                    },
                ]),
            },
            Requirement {
                name: "seconds_to_be_killed_by_mob",
                tiers: Tiers([
                    Tier {
                        name: "entry",
                        role: Role::Both {
                            dps: 10.0,
                            tank: 20,
                        },
                    },
                    Tier {
                        name: "mid",
                        role: Role::Both {
                            dps: 12.0,
                            tank: 40,
                        },
                    },
                    Tier {
                        name: "op",
                        role: Role::None,
                    },
                    Tier {
                        name: "high",
                        role: Role::None,
                    },
                ]),
            },
            Requirement {
                name: "seconds_to_be_killed_by_boss",
                tiers: Tiers([
                    Tier {
                        name: "high",
                        role: Role::Both { dps: 7.0, tank: 40 },
                    },
                    Tier {
                        name: "op",
                        role: Role::Both { dps: 9.0, tank: 80 },
                    },
                    Tier {
                        name: "mid",
                        role: Role::None,
                    },
                    Tier {
                        name: "entry",
                        role: Role::None,
                    },
                ]),
            },
        ])
    }
}
