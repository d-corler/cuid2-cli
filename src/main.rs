use clap::Parser;
use cuid2::CuidConstructor;

/// Simple program to generate cuid
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of ids to generate
    #[arg(short, long, default_value_t = 1)]
    rows: u8,

    /// Length of id(s)
    #[arg(short, long, default_value_t = 24)]
    length: u8,

    /// Clear console before output
    #[arg(short, long)]
    clear: bool,
}

fn main() {
    let args = Args::parse();

    if args.clear {
        print!("{esc}c", esc = 27 as char);
    }

    let constructor = CuidConstructor::new().with_length(args.length.into());

    for _ in 0..args.rows {
        println!("{}", constructor.create_id())
    }
}
