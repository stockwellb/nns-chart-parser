use nns_chart_parser::{
    chord::{Chord, ChordQuality},
    measure::Measure,
    parser::ChordParser,
    renderer::{ChordRenderer, SVG_HEIGHT, SVG_WIDTH},
};
use std::fs;
use std::path::PathBuf;

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(test_name: &str) -> Self {
        let path = PathBuf::from(format!("test_output_{}", test_name));
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }
        fs::create_dir(&path).unwrap();
        Self { path }
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

#[test]
fn test_render_single_chord() {
    let test_dir = TestDir::new("single_chord");
    let output_path = test_dir.path.join("output.svg");

    let chord = ChordParser::parse_file("tests/fixtures/triads/test_major.yaml").unwrap();

    let mut renderer = ChordRenderer::new();
    renderer.init_background().render_chord(&chord, 400, 200);

    renderer.save(output_path.to_str().unwrap()).unwrap();
    assert!(output_path.exists());

    // Check file is not empty
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(!content.is_empty());
    assert!(content.contains("<svg"));
    assert!(content.contains("</svg>"));
}

#[test]
fn test_render_multiple_chords() {
    let test_dir = TestDir::new("multiple_chords");
    let output_path = test_dir.path.join("output.svg");

    let chord1 = ChordParser::parse_file("tests/fixtures/triads/test_major.yaml").unwrap();
    let chord2 = ChordParser::parse_file("tests/fixtures/triads/test_minor.yaml").unwrap();

    let mut renderer = ChordRenderer::new();
    renderer
        .init_background()
        .render_chord(&chord1, 200, 200)
        .render_chord(&chord2, 600, 200);

    renderer.save(output_path.to_str().unwrap()).unwrap();

    // Check file content
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("1")); // First chord degree
    assert!(content.contains("m")); // Minor quality
}

#[test]
fn test_render_all_qualities() {
    let test_dir = TestDir::new("all_qualities");
    let output_path = test_dir.path.join("output.svg");

    let test_files = [
        ("tests/fixtures/triads/test_major.yaml", "1"),
        ("tests/fixtures/triads/test_minor.yaml", "m"),
        ("tests/fixtures/triads/test_sus2.yaml", "sus2"),
        ("tests/fixtures/triads/test_aug.yaml", "aug"),
        ("tests/fixtures/triads/test_dim.yaml", "dim"),
    ];

    let mut renderer = ChordRenderer::new();
    renderer.init_background();

    for (i, (file_path, _)) in test_files.iter().enumerate() {
        let chord = ChordParser::parse_file(file_path).unwrap();
        renderer.render_chord(&chord, 100 + i as i32 * 100, 200);
    }

    renderer.save(output_path.to_str().unwrap()).unwrap();

    // Check file content
    let content = fs::read_to_string(&output_path).unwrap();
    for (_, expected_text) in test_files {
        assert!(content.contains(expected_text));
    }
}

#[test]
fn test_render_at_boundaries() {
    let test_dir = TestDir::new("boundaries");
    let output_path = test_dir.path.join("output.svg");

    let chord = ChordParser::parse_file("tests/fixtures/triads/test_major.yaml").unwrap();

    let mut renderer = ChordRenderer::new();
    renderer
        .init_background()
        .render_chord(&chord, 0, 0) // Top-left
        .render_chord(&chord, SVG_WIDTH, SVG_HEIGHT); // Bottom-right

    renderer.save(output_path.to_str().unwrap()).unwrap();
    assert!(output_path.exists());
}

#[test]
fn test_save_to_invalid_path() {
    let chord = ChordParser::parse_file("tests/fixtures/triads/test_major.yaml").unwrap();

    let mut renderer = ChordRenderer::new();
    renderer.init_background().render_chord(&chord, 400, 200);

    let result = renderer.save("/invalid/path/test.svg");
    assert!(result.is_err());
}

#[test]
fn test_render_measure() {
    let test_dir = TestDir::new("measure");
    let output_path = test_dir.path.join("output.svg");

    let chords = vec![
        Chord {
            degree: 1,
            quality: ChordQuality::Major,
        },
        Chord {
            degree: 4,
            quality: ChordQuality::Major,
        },
        Chord {
            degree: 5,
            quality: ChordQuality::Major,
        },
    ];
    let measure = Measure::with_chords(chords);

    let mut renderer = ChordRenderer::new();
    renderer
        .init_background()
        .render_measure(&measure, 100, 200);

    renderer.save(output_path.to_str().unwrap()).unwrap();

    // Check file content
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("1")); // First chord
    assert!(content.contains("4")); // Second chord
    assert!(content.contains("5")); // Third chord
}
