use nns_chart_parser::{
    line::{ChordData, ChordDef, Line, LineElement, RepeatSign},
    renderer::ChordRenderer,
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
fn test_line_new() {
    let line = Line::new();
    assert!(line.line.is_empty());
}

#[test]
fn test_line_with_elements() {
    let chord_defs = vec![
        ChordDef {
            chord: ChordData {
                degree: 1,
                quality: "major".to_string(),
            },
        },
        ChordDef {
            chord: ChordData {
                degree: 4,
                quality: "major".to_string(),
            },
        },
    ];

    let elements = vec![
        LineElement::Repeat {
            repeat: RepeatSign::Begin,
        },
        LineElement::Measure {
            measure: chord_defs,
        },
        LineElement::Spacer,
        LineElement::Repeat {
            repeat: RepeatSign::End,
        },
    ];

    let line = Line::with_elements(elements.clone());
    assert_eq!(line.line.len(), 4);
}

#[test]
fn test_render_line() {
    let test_dir = TestDir::new("line");
    let output_path = test_dir.path.join("output.svg");

    let chord_defs1 = vec![
        ChordDef {
            chord: ChordData {
                degree: 1,
                quality: "major".to_string(),
            },
        },
        ChordDef {
            chord: ChordData {
                degree: 4,
                quality: "major".to_string(),
            },
        },
    ];

    let chord_defs2 = vec![
        ChordDef {
            chord: ChordData {
                degree: 5,
                quality: "major".to_string(),
            },
        },
        ChordDef {
            chord: ChordData {
                degree: 1,
                quality: "major".to_string(),
            },
        },
    ];

    let elements = vec![
        LineElement::Repeat {
            repeat: RepeatSign::Begin,
        },
        LineElement::Measure {
            measure: chord_defs1,
        },
        LineElement::Spacer,
        LineElement::Measure {
            measure: chord_defs2,
        },
        LineElement::Repeat {
            repeat: RepeatSign::End,
        },
    ];

    let line = Line::with_elements(elements);

    let mut renderer = ChordRenderer::new();
    renderer.init_background().render_line(&line, 100, 200);

    renderer.save(output_path.to_str().unwrap()).unwrap();

    // Check file content
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("<svg"));
    assert!(content.contains("</svg>"));
    assert!(content.contains("<circle")); // For spacer and repeat dots
    assert!(content.contains("<line")); // For repeat bars
}
