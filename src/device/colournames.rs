use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ColourName {
    #[serde(rename = "f5faf6")]
    White,
    #[serde(rename = "f1e0b5")]
    Warm,
    #[serde(rename = "efd275")]
    Glow,
    #[serde(rename = "4a418a")]
    Blue,
    #[serde(rename = "6c83ba")]
    LightBlue,
    #[serde(rename = "8f2686")]
    SaturatedPurple,
    #[serde(rename = "a9d62b")]
    Lime,
    #[serde(rename = "c984bb")]
    LightPurple,
    #[serde(rename = "d6e44b")]
    Yellow,
    #[serde(rename = "d9337c")]
    SaturatedPink,
    #[serde(rename = "da5d41")]
    DarkPeach,
    #[serde(rename = "dc4b31")]
    SaturatedRed,
    #[serde(rename = "dcf0f8")]
    ColdSky,
    #[serde(rename = "e491af")]
    Pink,
    #[serde(rename = "e57345")]
    Peach,
    #[serde(rename = "e78834")]
    WarmAmber,
    #[serde(rename = "e8bedd")]
    LightPink,
    #[serde(rename = "eaf6fb")]
    CoolDaylight,
    #[serde(rename = "ebb63e")]
    Candlelight,
    #[serde(rename = "efd275")]
    WarmGlow,
    #[serde(rename = "f1e0b5")]
    WarmWhite,
    #[serde(rename = "f2eccf")]
    Sunrise,
    #[serde(rename = "f5faf6")]
    CoolWhite,
}

impl std::default::Default for ColourName {
    fn default() -> Self {
        ColourName::Glow
    }
}
