use crate::chord::Chord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChordWrapper {
    chord: Chord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measure {
    #[serde(rename = "chord")]
    chords: Vec<ChordWrapper>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureCollection {
    pub measures: Vec<Measure>,
}

impl Measure {
    pub fn new() -> Self {
        Self { chords: Vec::new() }
    }

    pub fn with_chords(chords: Vec<Chord>) -> Self {
        Self {
            chords: chords
                .into_iter()
                .map(|c| ChordWrapper { chord: c })
                .collect(),
        }
    }

    pub fn add_chord(&mut self, chord: Chord) {
        self.chords.push(ChordWrapper { chord });
    }

    pub fn get_chords(&self) -> Vec<&Chord> {
        self.chords.iter().map(|w| &w.chord).collect()
    }

    pub fn get_chords_mut(&mut self) -> Vec<&mut Chord> {
        self.chords.iter_mut().map(|w| &mut w.chord).collect()
    }
}

impl MeasureCollection {
    pub fn new() -> Self {
        Self {
            measures: Vec::new(),
        }
    }

    pub fn with_measures(measures: Vec<Measure>) -> Self {
        Self { measures }
    }

    pub fn add_measure(&mut self, measure: Measure) {
        self.measures.push(measure);
    }
}
