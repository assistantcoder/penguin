use lalrpop::Configuration;
fn main(){
    Configuration::new()
        .use_cargo_dir_conventions()
        .process_dir("src/grammars")
        .unwrap()
}