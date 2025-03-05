use anyhow::{Context, Result};
use nns_chart_parser::{chord::Chord, parser::ChordParser, renderer::ChordRenderer};
use std::env;
use std::path::PathBuf;

fn print_usage() {
    eprintln!("Usage: nns-chart-parser <input.yaml> [output.svg]");
    eprintln!("If output.svg is not specified, will use input filename with .svg extension");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_path = if args.len() >= 3 {
        PathBuf::from(&args[2])
    } else {
        input_path.with_extension("svg")
    };

    // Parse the input YAML
    let chord = ChordParser::parse_file(&input_path)
        .with_context(|| format!("Failed to parse chord from {}", input_path.display()))?;

    // Create and render the SVG
    let mut renderer = ChordRenderer::new();
    renderer.init_background().render_chord(&chord, 400, 200); // Center the chord

    // Save the SVG
    renderer
        .save(output_path.to_str().unwrap())
        .with_context(|| format!("Failed to save SVG to {}", output_path.display()))?;

    println!("Successfully created {}", output_path.display());
    Ok(())
}
