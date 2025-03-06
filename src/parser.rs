use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::chord::Chord;
use crate::measure::{Measure, MeasureCollection};

pub struct ChordParser;
pub struct MeasureParser;
pub struct MeasureCollectionParser;

impl ChordParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Chord> {
        let yaml_content = fs::read_to_string(&path)
            .with_context(|| format!("No such file or directory: {}", path.as_ref().display()))?;

        // First try to parse as YAML
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&yaml_content)
            .with_context(|| "Failed to parse YAML: invalid format")?;

        // Check if we have a mapping
        if !yaml_value.is_mapping() {
            anyhow::bail!("Failed to parse YAML: invalid format");
        }

        // Then try to convert to our type
        let result: Result<Chord, serde_yaml::Error> = serde_yaml::from_value(yaml_value);
        match result {
            Ok(chord) => {
                // Validate degree is positive
                if chord.degree <= 0 {
                    anyhow::bail!("Chord degree must be positive, got: {}", chord.degree);
                }
                Ok(chord)
            }
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("missing field") {
                    anyhow::bail!("Failed to parse YAML: missing required fields");
                } else if msg.contains("unknown variant") {
                    anyhow::bail!("Failed to parse YAML: invalid field values");
                } else {
                    anyhow::bail!("Failed to parse YAML: {}", e);
                }
            }
        }
    }
}

impl MeasureParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Measure> {
        let yaml_content = fs::read_to_string(&path)
            .with_context(|| format!("No such file or directory: {}", path.as_ref().display()))?;

        // First try to parse as YAML
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&yaml_content)
            .with_context(|| "Failed to parse YAML: invalid format")?;

        // Check if we have a mapping
        if !yaml_value.is_mapping() {
            anyhow::bail!("Failed to parse YAML: invalid format");
        }

        // Then try to convert to our type
        let result: Result<Measure, serde_yaml::Error> = serde_yaml::from_value(yaml_value);
        match result {
            Ok(measure) => {
                // Validate all chords have positive degrees
                if let Some(invalid_chord) = measure.get_chords().iter().find(|c| c.degree <= 0) {
                    anyhow::bail!(
                        "Chord degree must be positive, got: {}",
                        invalid_chord.degree
                    );
                }
                Ok(measure)
            }
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("missing field") {
                    anyhow::bail!("Failed to parse YAML: missing required fields");
                } else if msg.contains("unknown variant") {
                    anyhow::bail!("Failed to parse YAML: invalid field values");
                } else {
                    anyhow::bail!("Failed to parse YAML: {}", e);
                }
            }
        }
    }
}

impl MeasureCollectionParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<MeasureCollection> {
        let yaml_content = fs::read_to_string(&path)
            .with_context(|| format!("No such file or directory: {}", path.as_ref().display()))?;

        // First try to parse as YAML
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&yaml_content)
            .with_context(|| "Failed to parse YAML: invalid format")?;

        // Check if we have a mapping
        if !yaml_value.is_mapping() {
            anyhow::bail!("Failed to parse YAML: invalid format");
        }

        // Then try to convert to our type
        let result: Result<MeasureCollection, serde_yaml::Error> =
            serde_yaml::from_value(yaml_value);
        match result {
            Ok(collection) => {
                // Validate all chords in all measures have positive degrees
                for (measure_idx, measure) in collection.measures.iter().enumerate() {
                    if let Some(invalid_chord) = measure.get_chords().iter().find(|c| c.degree <= 0)
                    {
                        anyhow::bail!(
                            "Chord degree must be positive, got: {} in measure {}",
                            invalid_chord.degree,
                            measure_idx + 1
                        );
                    }
                }
                Ok(collection)
            }
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("missing field") {
                    anyhow::bail!("Failed to parse YAML: missing required fields");
                } else if msg.contains("unknown variant") {
                    anyhow::bail!("Failed to parse YAML: invalid field values");
                } else {
                    anyhow::bail!("Failed to parse YAML: {}", e);
                }
            }
        }
    }
}
