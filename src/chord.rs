use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ChordQuality {
    #[serde(rename = "major")]
    Major,
    #[serde(rename = "minor")]
    Minor,
    #[serde(rename = "sus2")]
    Sus2,
    #[serde(rename = "sus4")]
    Sus4,
    #[serde(rename = "aug")]
    Aug,
    #[serde(rename = "dim")]
    Dim,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Chord {
    pub degree: i32,
    pub quality: ChordQuality,
}
