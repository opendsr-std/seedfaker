#[derive(Clone, Copy, Default, PartialEq)]
pub enum Script {
    #[default]
    Latin,
    Native,
    Both,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Ctx {
    #[default]
    None,
    Loose,
    Strict,
}

impl Ctx {
    pub fn parse(s: &str) -> Self {
        match s {
            "strict" => Self::Strict,
            "loose" => Self::Loose,
            _ => Self::None,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Corrupt {
    None,
    Low,
    Mid,
    High,
    Extreme,
}

impl Corrupt {
    pub fn parse_level(s: &str) -> Option<Self> {
        match s {
            "low" => Some(Self::Low),
            "mid" => Some(Self::Mid),
            "high" => Some(Self::High),
            "extreme" => Some(Self::Extreme),
            _ => None,
        }
    }

    pub fn rate(self) -> f64 {
        match self {
            Corrupt::None => 0.0,
            Corrupt::Low => 0.05,
            Corrupt::Mid => 0.15,
            Corrupt::High => 0.45,
            Corrupt::Extreme => 0.95,
        }
    }
}
