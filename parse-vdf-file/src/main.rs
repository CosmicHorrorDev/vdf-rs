use clap::Parser;
use keyvalues_parser::{text::parse::Opts, Vdf};

#[derive(Parser)]
struct Args {
    vdf_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { vdf_file } = Args::parse();

    let contents = std::fs::read_to_string(vdf_file)?;
    match Vdf::parse(&contents) {
        Ok(contents) => {
            println!("{:#?}", contents);
        }
        Err(err) => {
            eprintln!("Failed parsing with escaped characters: {:#?}", err);

            match Vdf::parse_with_opts(
                &contents,
                Opts {
                    parse_escaped_characters: false,
                },
            ) {
                Ok(contents) => {
                    println!("{:#?}", contents);
                }
                Err(err) => {
                    eprintln!("Failed parsing without escaped characters: {:#?}", err);
                    return Err(err.into());
                }
            }
        }
    }

    Ok(())
}
