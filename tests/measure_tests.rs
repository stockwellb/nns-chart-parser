use nns_chart_parser::{
    chord::{Chord, ChordQuality},
    measure::Measure,
};

#[test]
fn test_measure_new() {
    let measure = Measure::new();
    assert!(measure.get_chords().is_empty());
}

#[test]
fn test_measure_with_chords() {
    let chords = vec![
        Chord {
            degree: 1,
            quality: ChordQuality::Major,
        },
        Chord {
            degree: 4,
            quality: ChordQuality::Major,
        },
    ];
    let measure = Measure::with_chords(chords.clone());
    let measure_chords = measure.get_chords();
    assert_eq!(measure_chords.len(), chords.len());
    for (i, chord) in chords.iter().enumerate() {
        assert_eq!(measure_chords[i].degree, chord.degree);
        assert_eq!(measure_chords[i].quality, chord.quality);
    }
}

#[test]
fn test_measure_add_chord() {
    let mut measure = Measure::new();
    let chord = Chord {
        degree: 1,
        quality: ChordQuality::Major,
    };
    measure.add_chord(chord.clone());
    let measure_chords = measure.get_chords();
    assert_eq!(measure_chords.len(), 1);
    assert_eq!(measure_chords[0].degree, chord.degree);
    assert_eq!(measure_chords[0].quality, chord.quality);
}
