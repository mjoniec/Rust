// test 1
// this run creates exe and pdb files in folder
// rustc main.rs
// .\main.exe
// fn main() {
//     println!("Hello, world!");
// }

// test 2
// this run does not create any runnables in folder
// cargo run
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
