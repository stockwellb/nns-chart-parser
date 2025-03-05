use nns_chart_parser::{
    chord::{Chord, ChordQuality},
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

    let chord = Chord {
        degree: 1,
        quality: ChordQuality::Major,
    };

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

    let chord1 = Chord {
        degree: 1,
        quality: ChordQuality::Major,
    };
    let chord2 = Chord {
        degree: 5,
        quality: ChordQuality::Minor,
    };

    let mut renderer = ChordRenderer::new();
    renderer
        .init_background()
        .render_chord(&chord1, 200, 200)
        .render_chord(&chord2, 600, 200);

    renderer.save(output_path.to_str().unwrap()).unwrap();

    // Check file content
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("1")); // First chord degree
    assert!(content.contains("5")); // Second chord degree
}

#[test]
fn test_render_all_qualities() {
    let test_dir = TestDir::new("all_qualities");
    let output_path = test_dir.path.join("output.svg");

    let qualities = vec![
        ChordQuality::Major,
        ChordQuality::Minor,
        ChordQuality::Sus2,
        ChordQuality::Sus4,
        ChordQuality::Aug,
        ChordQuality::Dim,
    ];

    let mut renderer = ChordRenderer::new();
    renderer.init_background();

    for (i, quality) in qualities.into_iter().enumerate() {
        let chord = Chord {
            degree: (i + 1) as i32,
            quality,
        };
        renderer.render_chord(&chord, 100 + i as i32 * 100, 200);
    }

    renderer.save(output_path.to_str().unwrap()).unwrap();

    // Check file content
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("Major"));
    assert!(content.contains("Minor"));
    assert!(content.contains("Sus2"));
    assert!(content.contains("Sus4"));
    assert!(content.contains("Aug"));
    assert!(content.contains("Dim"));
}

#[test]
fn test_render_at_boundaries() {
    let test_dir = TestDir::new("boundaries");
    let output_path = test_dir.path.join("output.svg");

    let chord = Chord {
        degree: 1,
        quality: ChordQuality::Major,
    };

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
    let chord = Chord {
        degree: 1,
        quality: ChordQuality::Major,
    };

    let mut renderer = ChordRenderer::new();
    renderer.init_background().render_chord(&chord, 400, 200);

    let result = renderer.save("/invalid/path/test.svg");
    assert!(result.is_err());
}
