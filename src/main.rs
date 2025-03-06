use anyhow::{Context, Result};
use nns_chart_parser::{
    parser::ChordParser,
    renderer::{ChordRenderer, NotationType},
};
use std::env;
use std::path::PathBuf;

fn print_usage() {
    eprintln!("Usage: nns-chart-parser [--compact] <input.yaml> [output.svg]");
    eprintln!("Options:");
    eprintln!("  --compact    Use compact notation (1- for minor, 1+ for aug, 1º for dim)");
    eprintln!("If output.svg is not specified, will use input filename with .svg extension");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    // Parse arguments
    let mut compact = false;
    let mut input_path = None;
    let mut output_path = None;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--compact" => {
                compact = true;
                i += 1;
            }
            arg => {
                if input_path.is_none() {
                    input_path = Some(PathBuf::from(arg));
                } else if output_path.is_none() {
                    output_path = Some(PathBuf::from(arg));
                }
                i += 1;
            }
        }
    }

    let input_path = input_path.ok_or_else(|| {
        print_usage();
        anyhow::anyhow!("No input file specified")
    })?;

    let output_path = output_path.unwrap_or_else(|| input_path.with_extension("svg"));

    // Parse the input YAML
    let chord = ChordParser::parse_file(&input_path)
        .with_context(|| format!("Failed to parse chord from {}", input_path.display()))?;

    // Create and render the SVG
    let notation_type = if compact {
        NotationType::Compact
    } else {
        NotationType::Regular
    };

    let mut renderer = ChordRenderer::with_notation(notation_type);
    renderer.init_background().render_chord(&chord, 400, 200); // Center the chord

    // Save the SVG
    renderer
        .save(output_path.to_str().unwrap())
        .with_context(|| format!("Failed to save SVG to {}", output_path.display()))?;

    println!("Successfully created {}", output_path.display());
    Ok(())
}
