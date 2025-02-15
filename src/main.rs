use std::path::PathBuf;

pub mod compiler;
pub mod core;
pub mod lang;
#[cfg(test)]
mod test;

fn run(p: PathBuf) -> String {
    compiler::show_results(compiler::eval(compiler::compile(lang::parse(p))))
}

fn main() {
    let program = "main = S K K 3";
    println!(
        "{}",
        compiler::show_results(compiler::eval(compiler::compile(lang::parse_raw(
            program.to_string()
        ))))
    );
}
