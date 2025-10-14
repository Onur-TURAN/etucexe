mod utils;

fn input() {
    let mut input = String::new();  
    

}
fn main() {
    let result = utils::strings::extract_strings("program.exe").unwrap();

    for s in result {
        println!("{}", s);
    }
}