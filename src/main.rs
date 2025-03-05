use anyhow::Result;
use nns_chart_parser::{
    parser::ChordParser,
    renderer::{self, ChordRenderer},
};

fn main() -> Result<()> {
    // Parse the chord from YAML
    let chord = ChordParser::parse_file("input.yaml")?;

    // Create renderer and generate SVG
    let mut renderer = ChordRenderer::new();
    renderer.init_background().render_chord(
        &chord,
        renderer::SVG_WIDTH / 2,
        renderer::SVG_HEIGHT / 2,
    );

    // Save the result
    renderer.save("output.svg")?;
    println!("SVG has been generated as 'output.svg'");

    Ok(())
}
