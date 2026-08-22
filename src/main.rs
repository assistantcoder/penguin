use clap::{ Parser, Subcommand };
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
        Commands::Run { path: _ } => pnos_parser::parse(r#"fun main(name: string){
            string name = "isaac";
            }"#),
        _ => todo!()
    }

    
}