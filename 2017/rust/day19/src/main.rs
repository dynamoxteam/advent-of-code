extern crate common;

mod path;
use path::Path;

fn main() {
    let input = common::load_file_input("day19");
    let path = Path::load(input.as_str());
    let (letters, steps) = path.follow();

    println!("Collected letters: {}", letters);
    println!("Steps: {}", steps);
}

#[test]
fn test() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let input = String::new() +
        "     |          \n" +
        "     |  +--+    \n" +
        "     A  |  C    \n" +
        " F---|----E|--+ \n" +
        "     |  |  |  D \n" +
        "     +B-+  +--+ \n";

    let path = Path::load(input.as_str());
    let (letters, steps) = path.follow();

    assert_eq!(letters, "ABCDEF");
    assert_eq!(steps, 38);
}
