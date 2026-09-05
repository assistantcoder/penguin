use clap::{ Parser, Subcommand };
mod collector;
#[derive(Parser,Debug)]
struct Cli{
    #[command(subcommand)]
    commands: Commands
}

#[derive(Subcommand,Debug)]
enum Commands {
    ///run all your penguinos scripts on .pnos files with no build need
    Run{
        path: String
    },
    ///build all your penguinos scripts on .pnos files
    Build{
        path: String
    },
    ///check all your penguinos scripts on .pnos files
    Check{
        path: String
    }
}
fn main() {
    let cli = Cli::parse();
    println!("{cli:?}");
    match cli.commands {
        Commands::Run { path } => {
            match collector::read_path(path) {
                Ok(v) => {
                    pnos_codegen::codegen(pnos_parser::parse(v));
                    
                },
                Err(err) => println!("{err}")
            }
        },
        _ => todo!()
    }
}
