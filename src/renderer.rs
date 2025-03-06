use crate::chord::{Chord, ChordQuality};
use crate::line::{Line, LineElement, RepeatSign};
use crate::measure::Measure;
use anyhow::Result;
use svg::node::element::{Circle, Group, Line as SvgLine, Rectangle, Text};
use svg::node::Text as TextNode;
use svg::Document;

pub const SVG_WIDTH: i32 = 800;
pub const SVG_HEIGHT: i32 = 400;
pub const CHORD_SPACING: i32 = 100;
pub const LINE_HEIGHT: i32 = 100;
pub const REPEAT_DOT_SPACING: i32 = 6;
pub const REPEAT_LINE_SPACING: i32 = 3;
pub const SPACER_DOT_RADIUS: i32 = 3;

#[derive(Clone, Copy)]
pub enum NotationType {
    Regular,
    Compact,
}

pub struct ChordRenderer {
    document: Document,
    notation_type: NotationType,
}

impl ChordRenderer {
    pub fn new() -> Self {
        let document = Document::new()
            .set("width", SVG_WIDTH)
            .set("height", SVG_HEIGHT)
            .set("viewBox", (0, 0, SVG_WIDTH, SVG_HEIGHT));

        Self {
            document,
            notation_type: NotationType::Regular,
        }
    }

    pub fn with_notation(notation_type: NotationType) -> Self {
        let document = Document::new()
            .set("width", SVG_WIDTH)
            .set("height", SVG_HEIGHT)
            .set("viewBox", (0, 0, SVG_WIDTH, SVG_HEIGHT));

        Self {
            document,
            notation_type,
        }
    }

    pub fn render_chord(&mut self, chord: &Chord, x: i32, y: i32) -> &mut Self {
        let chord_group = self.create_chord_group(chord, x, y);
        self.document = self.document.clone().add(chord_group);
        self
    }

    pub fn render_measure(&mut self, measure: &Measure, x: i32, y: i32) -> &mut Self {
        let mut current_x = x;
        for chord in measure.get_chords() {
            self.render_chord(chord, current_x, y);
            current_x += CHORD_SPACING;
        }
        self
    }

    pub fn render_line(&mut self, line: &Line, x: i32, y: i32) -> &mut Self {
        let mut current_x = x;

        for element in &line.line {
            match element {
                LineElement::Measure { measure } => {
                    let measure = Measure::from(measure.clone());
                    self.render_measure(&measure, current_x, y);
                    current_x += CHORD_SPACING * measure.get_chords().len() as i32;
                }
                LineElement::Repeat { repeat } => {
                    self.render_repeat(repeat, current_x, y);
                    current_x += CHORD_SPACING / 2; // Half spacing for repeats
                }
                LineElement::Spacer => {
                    self.render_spacer(current_x, y);
                    current_x += CHORD_SPACING / 2; // Half spacing for spacers
                }
            }
        }
        self
    }

    pub fn render_repeat(&mut self, repeat_type: &RepeatSign, x: i32, y: i32) -> &mut Self {
        let mut group = Group::new();
        let center_y = y;

        // Add two vertical lines
        let line1 = SvgLine::new()
            .set("x1", x)
            .set("y1", center_y - LINE_HEIGHT / 3)
            .set("x2", x)
            .set("y2", center_y + LINE_HEIGHT / 3)
            .set("stroke", "black")
            .set("stroke-width", 2);

        let line2 = SvgLine::new()
            .set("x1", x + REPEAT_LINE_SPACING)
            .set("y1", center_y - LINE_HEIGHT / 3)
            .set("x2", x + REPEAT_LINE_SPACING)
            .set("y2", center_y + LINE_HEIGHT / 3)
            .set("stroke", "black")
            .set("stroke-width", 2);

        group = group.add(line1).add(line2);

        // Add dots based on repeat type
        let dot_x = match repeat_type {
            RepeatSign::Begin => x + REPEAT_LINE_SPACING + REPEAT_DOT_SPACING,
            RepeatSign::End => x - REPEAT_DOT_SPACING,
        };

        let dot1 = Circle::new()
            .set("cx", dot_x)
            .set("cy", center_y - REPEAT_DOT_SPACING)
            .set("r", 2)
            .set("fill", "black");

        let dot2 = Circle::new()
            .set("cx", dot_x)
            .set("cy", center_y + REPEAT_DOT_SPACING)
            .set("r", 2)
            .set("fill", "black");

        group = group.add(dot1).add(dot2);
        self.document = self.document.clone().add(group);
        self
    }

    pub fn render_spacer(&mut self, x: i32, y: i32) -> &mut Self {
        let dot = Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", SPACER_DOT_RADIUS)
            .set("fill", "black");

        self.document = self.document.clone().add(dot);
        self
    }

    pub fn save(&self, path: &str) -> Result<()> {
        svg::save(path, &self.document).map_err(|e| anyhow::anyhow!("Failed to save SVG: {}", e))
    }

    pub fn init_background(&mut self) -> &mut Self {
        let background = Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "white");
        self.document = self.document.clone().add(background);
        self
    }

    fn quality_to_string(&self, quality: &ChordQuality) -> String {
        match (quality, self.notation_type) {
            (ChordQuality::Major, _) => String::new(),
            (ChordQuality::Minor, NotationType::Regular) => "m".to_string(),
            (ChordQuality::Minor, NotationType::Compact) => "-".to_string(),
            (ChordQuality::Sus2, _) => "sus2".to_string(),
            (ChordQuality::Sus4, _) => "sus4".to_string(),
            (ChordQuality::Aug, NotationType::Regular) => "aug".to_string(),
            (ChordQuality::Aug, NotationType::Compact) => "+".to_string(),
            (ChordQuality::Dim, NotationType::Regular) => "dim".to_string(),
            (ChordQuality::Dim, NotationType::Compact) => "º".to_string(),
        }
    }

    fn create_chord_group(&self, chord: &Chord, x: i32, y: i32) -> Group {
        let mut group = Group::new();

        // Create text element for the chord
        let chord_text = format!("{}{}", chord.degree, self.quality_to_string(&chord.quality));
        let text = Text::new()
            .set("x", x)
            .set("y", y)
            .set("text-anchor", "middle")
            .set("dominant-baseline", "middle")
            .set("font-family", "Arial")
            .set("font-size", 20)
            .add(TextNode::new(chord_text));

        group = group.add(text);
        group
    }
}
