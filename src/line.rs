use crate::measure::Measure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RepeatSign {
    Begin,
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum LineElement {
    Measure {
        #[serde(rename = "content")]
        measure: Vec<ChordDef>,
    },
    Repeat {
        #[serde(rename = "content")]
        repeat: RepeatSign,
    },
    Spacer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChordDef {
    pub chord: ChordData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChordData {
    pub degree: i32,
    pub quality: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub line: Vec<LineElement>,
}

impl Line {
    pub fn new() -> Self {
        Self { line: Vec::new() }
    }

    pub fn with_elements(elements: Vec<LineElement>) -> Self {
        Self { line: elements }
    }

    pub fn add_element(&mut self, element: LineElement) {
        self.line.push(element);
    }
}

impl From<ChordData> for crate::chord::Chord {
    fn from(data: ChordData) -> Self {
        use crate::chord::ChordQuality;
        let quality = match data.quality.as_str() {
            "major" => ChordQuality::Major,
            "minor" => ChordQuality::Minor,
            "sus2" => ChordQuality::Sus2,
            "sus4" => ChordQuality::Sus4,
            "aug" => ChordQuality::Aug,
            "dim" => ChordQuality::Dim,
            _ => ChordQuality::Major, // Default to major if unknown
        };
        Self {
            degree: data.degree,
            quality,
        }
    }
}

impl From<Vec<ChordDef>> for Measure {
    fn from(defs: Vec<ChordDef>) -> Self {
        let chords = defs.into_iter().map(|def| def.chord.into()).collect();
        Self::with_chords(chords)
    }
}
