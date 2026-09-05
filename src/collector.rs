use std::{env, fs};
pub fn read_path(path: String) -> Result<String,std::io::Error>{
    let mut v = env::current_exe()?;
    v.push(&path);
    println!("{v:?}");
    let output = fs::read_to_string(&path)?;
    Ok(output)
}

