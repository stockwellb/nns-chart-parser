use nns_chart_parser::chord::{Chord, ChordQuality};
use serde_yaml;

#[test]
fn test_chord_quality_serialization() {
    // Test all chord qualities
    let qualities = vec![
        ChordQuality::Major,
        ChordQuality::Minor,
        ChordQuality::Sus2,
        ChordQuality::Sus4,
        ChordQuality::Aug,
        ChordQuality::Dim,
    ];

    for quality in qualities {
        let serialized = serde_yaml::to_string(&quality).unwrap();
        let deserialized: ChordQuality = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(format!("{:?}", quality), format!("{:?}", deserialized));
    }
}

#[test]
fn test_chord_serialization() {
    let chord = Chord {
        degree: 1,
        quality: ChordQuality::Major,
    };

    let serialized = serde_yaml::to_string(&chord).unwrap();
    let deserialized: Chord = serde_yaml::from_str(&serialized).unwrap();

    assert_eq!(chord.degree, deserialized.degree);
    assert_eq!(
        format!("{:?}", chord.quality),
        format!("{:?}", deserialized.quality)
    );
}

#[test]
fn test_chord_debug_format() {
    let chord = Chord {
        degree: 1,
        quality: ChordQuality::Major,
    };

    assert_eq!(
        format!("{:?}", chord),
        "Chord { degree: 1, quality: Major }"
    );
}
