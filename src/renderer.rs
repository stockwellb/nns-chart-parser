use crate::chord::Chord;
use anyhow::Result;
use serde_yaml;
use svg::node::element::{Circle, Group, Rectangle, Text};
use svg::node::Text as TextNode;
use svg::Document;

pub const SVG_WIDTH: i32 = 800;
pub const SVG_HEIGHT: i32 = 400;
pub const CIRCLE_RADIUS: i32 = 30;

pub struct ChordRenderer {
    document: Document,
}

impl ChordRenderer {
    pub fn new() -> Self {
        let document = Document::new()
            .set("width", SVG_WIDTH)
            .set("height", SVG_HEIGHT)
            .set("viewBox", (0, 0, SVG_WIDTH, SVG_HEIGHT));

        Self { document }
    }

    pub fn render_chord(&mut self, chord: &Chord, x: i32, y: i32) -> &mut Self {
        let chord_group = self.create_chord_group(chord, x, y);
        self.document = self.document.clone().add(chord_group);
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

    fn create_chord_group(&self, chord: &Chord, x: i32, y: i32) -> Group {
        let mut group = Group::new();

        // Draw circle for the chord
        let circle = Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", CIRCLE_RADIUS)
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 2);

        // Add text for degree
        let degree_text = Text::new()
            .set("x", x)
            .set("y", y)
            .set("text-anchor", "middle")
            .set("dominant-baseline", "middle")
            .set("font-family", "Arial")
            .set("font-size", 20)
            .add(TextNode::new(chord.degree.to_string()));

        // Add text for quality
        let quality_str = serde_yaml::to_string(&chord.quality)
            .unwrap_or_else(|_| "Unknown".to_string())
            .trim()
            .to_string();

        let quality_str = quality_str
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_uppercase().next().unwrap_or(c)
                } else {
                    c
                }
            })
            .collect::<String>();

        let quality_text = Text::new()
            .set("x", x)
            .set("y", y + CIRCLE_RADIUS + 20)
            .set("text-anchor", "middle")
            .set("font-family", "Arial")
            .set("font-size", 14)
            .add(TextNode::new(quality_str));

        group = group.add(circle).add(degree_text).add(quality_text);
        group
    }
}
