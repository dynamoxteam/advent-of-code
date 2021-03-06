extern crate common;

mod sequence;

use sequence::Sequence;
use sequence::Template;

fn main() {
    let input = common::load_file_input("day16");
    let template = Template::parse(input.as_str(), 16);

    println!(
        "Programs after dance: {}",
        Sequence::identity(16) + &template
    );

    println!(
        "Programs after billionth dance: {}",
        Sequence::identity(16) + 1_000_000_000 * &template
    );
}

#[test]
fn test_moves() {
    let template = Template::parse("s1,x3/4,pe/b", 5);
    let mut sequence = Sequence::identity(5);

    sequence += &template;
    assert_eq!(format!("{}", sequence), "baedc");

    sequence += &template;
    assert_eq!(format!("{}", sequence), "ceadb");

    sequence += &template;
    assert_eq!(format!("{}", sequence), "ecbda");

    sequence += &template;
    assert_eq!(format!("{}", sequence), "abcde");

    sequence += &template;
    assert_eq!(format!("{}", sequence), "baedc");

    sequence += &template;
    assert_eq!(format!("{}", sequence), "ceadb");

    sequence += &template;
    assert_eq!(format!("{}", sequence), "ecbda");

    assert_eq!(format!("{}", Sequence::identity(5) + &template), "baedc");
    
    assert_eq!(
        format!("{}", Sequence::identity(5) + &template * 2),
        "ceadb"
    );
    
    assert_eq!(
        format!("{}", Sequence::identity(5) + &template * 3),
        "ecbda"
    );
    
    assert_eq!(
        format!("{}", Sequence::identity(5) + &template * 4),
        "abcde"
    );
    
    assert_eq!(
        format!("{}", Sequence::identity(5) + &template * 5),
        "baedc"
    );
    
    assert_eq!(
        format!("{}", Sequence::identity(5) + &template * 6),
        "ceadb"
    );
    
    assert_eq!(
        format!("{}", Sequence::identity(5) + &template * 7),
        "ecbda"
    );
}
