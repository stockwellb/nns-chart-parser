use anyhow::Result;
use nns_chart_parser::chord::ChordQuality;
use nns_chart_parser::parser::ChordParser;

#[test]
fn test_parse_valid_chord() -> Result<()> {
    let chord = ChordParser::parse_file("tests/fixtures/triads/test_minor.yaml")?;
    assert_eq!(chord.degree, 1);
    assert_eq!(chord.quality, ChordQuality::Minor);
    Ok(())
}

#[test]
fn test_parse_invalid_chord() {
    let result = ChordParser::parse_file("tests/fixtures/invalid_chords/invalid_chord.yaml");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Failed to parse YAML"));
}

#[test]
fn test_parse_all_qualities() -> Result<()> {
    let test_files = [
        ("tests/fixtures/triads/test_major.yaml", ChordQuality::Major),
        ("tests/fixtures/triads/test_minor.yaml", ChordQuality::Minor),
        ("tests/fixtures/triads/test_sus2.yaml", ChordQuality::Sus2),
        ("tests/fixtures/triads/test_sus4.yaml", ChordQuality::Sus4),
        ("tests/fixtures/triads/test_aug.yaml", ChordQuality::Aug),
        ("tests/fixtures/triads/test_dim.yaml", ChordQuality::Dim),
    ];

    for (file_path, expected_quality) in test_files {
        let chord = ChordParser::parse_file(file_path)?;
        assert_eq!(chord.quality, expected_quality);
    }
    Ok(())
}

#[test]
fn test_parse_missing_fields() {
    let result = ChordParser::parse_file("tests/fixtures/invalid_chords/missing_fields.yaml");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("missing required fields"));
}

#[test]
fn test_parse_invalid_degree() {
    let result = ChordParser::parse_file("tests/fixtures/invalid_chords/invalid_degree.yaml");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("must be positive"));
}

#[test]
fn test_parse_invalid_quality() {
    let result = ChordParser::parse_file("tests/fixtures/invalid_chords/invalid_quality.yaml");
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
