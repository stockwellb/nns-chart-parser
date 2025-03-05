use anyhow::Result;
use nns_chart_parser::chord::{Chord, ChordQuality};
use nns_chart_parser::parser::ChordParser;
use serde::Deserialize;
use std::fs;

#[test]
fn test_parse_valid_chord() -> Result<()> {
    let chord = ChordParser::parse_file("tests/fixtures/valid_chord.yaml")?;
    assert_eq!(chord.degree, 3);
    assert_eq!(chord.quality, ChordQuality::Minor);
    Ok(())
}

#[test]
fn test_parse_invalid_chord() {
    let result = ChordParser::parse_file("tests/fixtures/invalid_chord.yaml");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Failed to parse YAML"));
}

#[test]
fn test_parse_all_qualities() -> Result<()> {
    let content = fs::read_to_string("tests/fixtures/all_qualities.yaml")?;
    let docs: Vec<_> = serde_yaml::Deserializer::from_str(&content)
        .map(|d| serde_yaml::Value::deserialize(d))
        .collect::<Result<_, _>>()?;

    let expected_qualities = vec![
        ChordQuality::Major,
        ChordQuality::Minor,
        ChordQuality::Sus2,
        ChordQuality::Sus4,
        ChordQuality::Aug,
        ChordQuality::Dim,
    ];

    for (i, doc) in docs.iter().enumerate() {
        let chord: Chord = serde_yaml::from_value(doc.clone())?;
        assert_eq!(chord.quality, expected_qualities[i]);
    }
    Ok(())
}

#[test]
fn test_parse_missing_fields() {
    let result = ChordParser::parse_file("tests/fixtures/missing_fields.yaml");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("missing required fields"));
}

#[test]
fn test_parse_invalid_degree() {
    let result = ChordParser::parse_file("tests/fixtures/invalid_degree.yaml");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("must be positive"));
}

#[test]
fn test_parse_invalid_quality() {
    let result = ChordParser::parse_file("tests/fixtures/invalid_quality.yaml");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("invalid field values"));
}

#[test]
fn test_parse_nonexistent_file() {
    let result = ChordParser::parse_file("tests/fixtures/nonexistent.yaml");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No such file or directory"));
}
