mod utils;

fn main() {
    let result = utils::strings::extract_strings("program.exe").unwrap();

    for s in result {
        println!("{}", s);
    }
}