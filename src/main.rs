use anyhow::Result;
use clap::Parser;
use nns_chart_parser::{
    parser::LineParser,
    renderer::{ChordRenderer, NotationType},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input YAML file
    input_file: String,

    /// Use compact notation
    #[arg(long)]
    compact: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let line = LineParser::parse_file(&args.input_file)?;

    let notation_type = if args.compact {
        NotationType::Compact
    } else {
        NotationType::Regular
    };

    let mut renderer = ChordRenderer::with_notation(notation_type);
    renderer.init_background().render_line(&line, 100, 200);

    let output_path = format!("{}.svg", args.input_file);
    renderer.save(&output_path)?;

    Ok(())
}
