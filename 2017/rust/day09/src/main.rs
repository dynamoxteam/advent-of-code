extern crate common;

fn process_stream(input: &str) -> (usize, usize) {
    let mut stream = input.chars();

    process_group(&mut stream, 0)
}

fn process_group<I: Iterator<Item = char>>(stream: &mut I, base_score: usize) -> (usize, usize) {
    let mut score = base_score;
    let mut garbage_chars = 0;

    while let Some(c) = stream.next() {
        match c {
            '{' => {
                let (score_inc, chars_inc) = process_group(stream, base_score + 1);

                score += score_inc;
                garbage_chars += chars_inc;
            }
            '}' => return (score, garbage_chars),
            '<' => garbage_chars += process_garbage(stream),
            '!' => {
                stream.next();
            }
            _ => (),
        };
    }

    (score, garbage_chars)
}

fn process_garbage<I: Iterator<Item = char>>(stream: &mut I) -> usize {
    let mut chars = 0;

    while let Some(c) = stream.next() {
        match c {
            '>' => return chars,
            '!' => {
                stream.next();
            }
            _ => chars += 1,
        };
    }

    chars
}

fn main() {
    let input = common::load_file_input("day09");
    let (score, garbage_chars) = process_stream(input.as_str());

    println!("Total score: {}", score);
    println!("Garbage characters: {}", garbage_chars);
}

#[test]
fn test_process_stream() {
    assert_eq!(process_stream("{}"), (1, 0));
    assert_eq!(process_stream("{{{}}}"), (6, 0));
    assert_eq!(process_stream("{{},{}}"), (5, 0));
    assert_eq!(process_stream("{{{},{},{{}}}}"), (16, 0));
    assert_eq!(process_stream("{<a>,<a>,<a>,<a>}"), (1, 4));
    assert_eq!(process_stream("{{<ab>},{<ab>},{<ab>},{<ab>}}"), (9, 8));
    assert_eq!(process_stream("{{<!!>},{<!!>},{<!!>},{<!!>}}"), (9, 0));
    assert_eq!(process_stream("{{<a!>},{<a!>},{<a!>},{<ab>}}"), (3, 17));
}

#[test]
fn test_process_garbage() {
    assert_eq!(process_stream("<>"), (0, 0));
    assert_eq!(process_stream("<random characters>"), (0, 17));
    assert_eq!(process_stream("<<<<>"), (0, 3));
    assert_eq!(process_stream("<{!>}>"), (0, 2));
    assert_eq!(process_stream("<!!>"), (0, 0));
    assert_eq!(process_stream("<!!!>>"), (0, 0));
    assert_eq!(process_stream(r#"<{o"i!a,<{i<a>"#), (0, 10));
}
