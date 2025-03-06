use anyhow::Result;
use nns_chart_parser::{
    chord::ChordQuality,
    parser::{ChordParser, MeasureCollectionParser, MeasureParser},
};

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

#[test]
fn test_parse_measure() {
    let measure = MeasureParser::parse_file("tests/fixtures/measures/test_measure.yaml").unwrap();
    let chords = measure.get_chords();
    assert_eq!(chords.len(), 3);
    assert_eq!(chords[0].degree, 1);
    assert_eq!(chords[0].quality, ChordQuality::Major);
    assert_eq!(chords[1].degree, 4);
    assert_eq!(chords[1].quality, ChordQuality::Major);
    assert_eq!(chords[2].degree, 5);
    assert_eq!(chords[2].quality, ChordQuality::Major);
}

#[test]
fn test_parse_invalid_measure() {
    let result = MeasureParser::parse_file("tests/fixtures/measures/nonexistent.yaml");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No such file or directory"));
}

#[test]
fn test_parse_measure_with_invalid_chord() {
    let result = MeasureParser::parse_file("tests/fixtures/measures/test_invalid_chord.yaml");
    assert!(result.is_err());
}

#[test]
fn test_parse_multiple_measures() {
    let collection =
        MeasureCollectionParser::parse_file("tests/fixtures/measures/test_multiple_measures.yaml")
            .unwrap();

    assert_eq!(collection.measures.len(), 3);

    // Check first measure (I-IV-V)
    let chords = collection.measures[0].get_chords();
    assert_eq!(chords.len(), 3);
    assert_eq!(chords[0].degree, 1);
    assert_eq!(chords[0].quality, ChordQuality::Major);
    assert_eq!(chords[1].degree, 4);
    assert_eq!(chords[2].degree, 5);

    // Check second measure (vi-ii-V-I)
    let chords = collection.measures[1].get_chords();
    assert_eq!(chords.len(), 4);
    assert_eq!(chords[0].degree, 6);
    assert_eq!(chords[0].quality, ChordQuality::Minor);
    assert_eq!(chords[1].degree, 2);
    assert_eq!(chords[1].quality, ChordQuality::Minor);
    assert_eq!(chords[2].degree, 5);
    assert_eq!(chords[3].degree, 1);

    // Check third measure (IV-V)
    let chords = collection.measures[2].get_chords();
    assert_eq!(chords.len(), 2);
    assert_eq!(chords[0].degree, 4);
    assert_eq!(chords[1].degree, 5);
}

#[test]
fn test_parse_invalid_multiple_measures() {
    let result = MeasureCollectionParser::parse_file(
        "tests/fixtures/measures/test_invalid_multiple_measures.yaml",
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("must be positive"));
}

#[test]
fn test_parse_nonexistent_multiple_measures() {
    let result = MeasureCollectionParser::parse_file("tests/fixtures/measures/nonexistent.yaml");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No such file or directory"));
}
