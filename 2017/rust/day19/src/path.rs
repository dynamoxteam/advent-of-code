trait NorthSouth {
    fn get_north(&self) -> Option<usize>;
    fn get_south(&self) -> Option<usize>;

    fn set_north(&mut self, node: Option<usize>);
    fn set_south(&mut self, node: Option<usize>);
}

trait EastWest {
    fn get_east(&self) -> Option<usize>;
    fn get_west(&self) -> Option<usize>;

    fn set_east(&mut self, node: Option<usize>);
    fn set_west(&mut self, node: Option<usize>);
}

trait Length {
    fn get_length(&self) -> usize;
}

#[derive(Clone, Debug)]
struct NsSection {
    length: usize,
    north: Option<usize>,
    south: Option<usize>,
}

#[derive(Clone, Debug)]
struct EwSection {
    length: usize,
    east: Option<usize>,
    west: Option<usize>,
}

#[derive(Clone, Debug)]
struct Letter {
    letter: char,
    north: Option<usize>,
    east: Option<usize>,
    south: Option<usize>,
    west: Option<usize>,
}

#[derive(Clone, Debug)]
struct Corner {
    north: Option<usize>,
    east: Option<usize>,
    south: Option<usize>,
    west: Option<usize>,
}

#[derive(Clone, Debug)]
enum Node {
    NsSection(NsSection),
    EwSection(EwSection),
    Letter(Letter),
    Corner(Corner),
}

#[derive(Clone, Debug)]
pub struct Path {
    nodes: Vec<Node>,
    start: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

macro_rules! implement_ns {
    ($T:ty) => {
        impl NorthSouth for $T {
            fn get_north(&self) -> Option<usize> {
                self.north
            }

            fn get_south(&self) -> Option<usize> {
                self.south
            }

            fn set_north(&mut self, node: Option<usize>) {
                self.north = node;
            }

            fn set_south(&mut self, node: Option<usize>) {
                self.south = node;
            }
        }
    }
}

macro_rules! implement_ew {
    ($T:ty) => {
        impl EastWest for $T {
            fn get_east(&self) -> Option<usize> {
                self.east
            }

            fn get_west(&self) -> Option<usize> {
                self.west
            }

            fn set_east(&mut self, node: Option<usize>) {
                self.east = node;
            }

            fn set_west(&mut self, node: Option<usize>) {
                self.west = node;
            }
        }
    }
}

implement_ns!(NsSection);
implement_ns!(Letter);
implement_ns!(Corner);

implement_ew!(EwSection);
implement_ew!(Letter);
implement_ew!(Corner);

macro_rules! implement_section_length {
    ($T:ty) => {
        impl Length for $T {
            fn get_length(&self) -> usize {
                self.length
            }
        }
    }
}

implement_section_length!(NsSection);
implement_section_length!(EwSection);

macro_rules! implement_std_length {
    ($T:ty) => {
        impl Length for $T {
            fn get_length(&self) -> usize {
                1
            }
        }
    }
}

implement_std_length!(Letter);
implement_std_length!(Corner);

impl Length for Node {
    fn get_length(&self) -> usize {
        match *self {
            Node::NsSection(ref section) => section.get_length(),
            Node::EwSection(ref section) => section.get_length(),
            Node::Letter(ref letter) => letter.get_length(),
            Node::Corner(ref corner) => corner.get_length(),
        }
    }
}

impl NsSection {
    pub fn new() -> NsSection {
        NsSection {
            length: 1,
            north: None,
            south: None,
        }
    }
}

impl EwSection {
    pub fn new() -> EwSection {
        EwSection {
            length: 1,
            east: None,
            west: None,
        }
    }
}

impl Letter {
    pub fn new(letter: char) -> Letter {
        Letter {
            letter,
            north: None,
            east: None,
            south: None,
            west: None,
        }
    }
}

impl Corner {
    pub fn new() -> Corner {
        Corner {
            north: None,
            east: None,
            south: None,
            west: None,
        }
    }
}

impl Node {
    pub fn get_ns(&self) -> Option<&NorthSouth> {
        match *self {
            Node::NsSection(ref section) => Some(section),
            Node::Letter(ref letter) => Some(letter),
            Node::Corner(ref corner) => Some(corner),
            _ => None,
        }
    }

    pub fn get_ns_mut(&mut self) -> Option<&mut NorthSouth> {
        match *self {
            Node::NsSection(ref mut section) => Some(section),
            Node::Letter(ref mut letter) => Some(letter),
            Node::Corner(ref mut corner) => Some(corner),
            _ => None,
        }
    }

    pub fn get_ew(&self) -> Option<&EastWest> {
        match *self {
            Node::EwSection(ref section) => Some(section),
            Node::Letter(ref letter) => Some(letter),
            Node::Corner(ref corner) => Some(corner),
            _ => None,
        }
    }

    pub fn get_ew_mut(&mut self) -> Option<&mut EastWest> {
        match *self {
            Node::EwSection(ref mut section) => Some(section),
            Node::Letter(ref mut letter) => Some(letter),
            Node::Corner(ref mut corner) => Some(corner),
            _ => None,
        }
    }

    pub fn get_letter(&self) -> Option<&Letter> {
        match *self {
            Node::Letter(ref letter) => Some(letter),
            _ => None,
        }
    }

    pub fn next(&self, heading: Direction) -> (Option<usize>, Direction) {
        match heading {
            Direction::North | Direction::South => {
                let ns = self.get_ns();
                let ahead = ns.and_then(|ns| {
                    if heading == Direction::North {
                        ns.get_north()
                    } else {
                        ns.get_south()
                    }
                });

                if ahead.is_some() {
                    return (ahead, heading);
                }

                let ew = self.get_ew();
                let east = ew.and_then(|ew| ew.get_east());
                let west = ew.and_then(|ew| ew.get_west());

                if east.is_some() && west.is_some() {
                    (None, heading)
                } else if east.is_some() {
                    (east, Direction::East)
                } else if west.is_some() {
                    (west, Direction::West)
                } else {
                    (None, heading)
                }
            }
            Direction::East | Direction::West => {
                let ew = self.get_ew();
                let ahead = ew.and_then(|ew| {
                    if heading == Direction::East {
                        ew.get_east()
                    } else {
                        ew.get_west()
                    }
                });

                if ahead.is_some() {
                    return (ahead, heading);
                }

                let ns = self.get_ns();
                let north = ns.and_then(|ns| ns.get_north());
                let south = ns.and_then(|ns| ns.get_south());

                if north.is_some() && south.is_some() {
                    (None, heading)
                } else if north.is_some() {
                    (north, Direction::North)
                } else if south.is_some() {
                    (south, Direction::South)
                } else {
                    (None, heading)
                }
            }
        }
    }
}

impl Path {
    pub fn load(input: &str) -> Path {
        let mut nodes: Vec<Node> = Vec::new();
        let mut start: Option<usize> = None;
        let mut upper_indexes: Vec<Option<usize>> = Vec::new();

        for line in input.lines() {
            let mut left_index: Option<usize> = None;

            for (i, c) in line.chars().enumerate() {
                if upper_indexes.len() <= i {
                    upper_indexes.push(None);
                }

                let mut upper_index = upper_indexes.get_mut(i).unwrap();

                match c {
                    '|' => {
                        Path::load_ns(false, &mut upper_index, &mut nodes, &mut start);
                        Path::load_ew(true, &mut left_index, &mut nodes);
                    }
                    '-' => {
                        Path::load_ew(false, &mut left_index, &mut nodes);
                        Path::load_ns(true, &mut upper_index, &mut nodes, &mut start);
                    }
                    '+' => Path::load_corner(&mut left_index, &mut upper_index, &mut nodes),
                    'A'...'Z' => {
                        Path::load_letter(c, &mut left_index, &mut upper_index, &mut nodes)
                    }
                    _ => {
                        left_index = None;
                        *upper_index = None;
                    }
                }
            }
        }

        Path { nodes, start }
    }

    fn load_ns(
        extend: bool,
        upper_index: &mut Option<usize>,
        nodes: &mut Vec<Node>,
        start: &mut Option<usize>,
    ) {
        {
            let upper_node = upper_index.and_then(|n| nodes.get_mut(n));

            if let Some(&mut Node::NsSection(ref mut ns_section)) = upper_node {
                ns_section.length += 1;
                return;
            }
        }

        let next_index = nodes.len();
        let mut section = NsSection::new();

        {
            let upper_node = upper_index.and_then(|n| nodes.get_mut(n));

            if let Some(upper_ns) = upper_node.and_then(|n| n.get_ns_mut()) {
                upper_ns.set_south(Some(next_index));
                section.north = *upper_index;
            } else if extend {
                return;
            } else if start.is_none() {
                *start = Some(next_index);
            }
        }

        nodes.push(Node::NsSection(section));
        *upper_index = Some(next_index);
    }

    fn load_ew(extend: bool, left_index: &mut Option<usize>, nodes: &mut Vec<Node>) {
        {
            let left_node = left_index.and_then(|n| nodes.get_mut(n));

            if let Some(&mut Node::EwSection(ref mut ew_section)) = left_node {
                ew_section.length += 1;
                return;
            }
        }

        let next_index = nodes.len();
        let mut section = EwSection::new();

        {
            let left_node = left_index.and_then(|n| nodes.get_mut(n));

            if let Some(left_ew) = left_node.and_then(|n| n.get_ew_mut()) {
                left_ew.set_east(Some(next_index));
                section.west = *left_index;
            } else if extend {
                return;
            }
        }

        nodes.push(Node::EwSection(section));
        *left_index = Some(next_index);
    }

    fn preload_nsew<T>(
        nsew: &mut T,
        left_index: &mut Option<usize>,
        upper_index: &mut Option<usize>,
        nodes: &mut Vec<Node>,
    ) where
        T: NorthSouth + EastWest,
    {
        let next_index = nodes.len();

        {
            let left_node = left_index.and_then(|n| nodes.get_mut(n));

            if let Some(left_ew) = left_node.and_then(|n| n.get_ew_mut()) {
                left_ew.set_east(Some(next_index));
                nsew.set_west(*left_index);
            }
        }

        {
            let upper_node = upper_index.and_then(|n| nodes.get_mut(n));

            if let Some(upper_ns) = upper_node.and_then(|n| n.get_ns_mut()) {
                upper_ns.set_south(Some(next_index));
                nsew.set_north(*upper_index);
            }
        }

        *left_index = Some(next_index);
        *upper_index = Some(next_index);
    }

    fn load_corner(
        left_index: &mut Option<usize>,
        upper_index: &mut Option<usize>,
        nodes: &mut Vec<Node>,
    ) {
        let mut corner = Corner::new();

        Path::preload_nsew(&mut corner, left_index, upper_index, nodes);
        nodes.push(Node::Corner(corner));
    }

    fn load_letter(
        letter: char,
        left_index: &mut Option<usize>,
        upper_index: &mut Option<usize>,
        nodes: &mut Vec<Node>,
    ) {
        let mut letter = Letter::new(letter);

        Path::preload_nsew(&mut letter, left_index, upper_index, nodes);
        nodes.push(Node::Letter(letter));
    }

    pub fn follow(&self) -> (String, usize) {
        let mut letters = String::new();
        let mut index = self.start;
        let mut direction = Direction::South;
        let mut steps = 0;

        while let Some(node) = index.and_then(|n| self.nodes.get(n)) {
            let letter = node.get_letter().map(|l| l.letter);

            if let Some(letter) = letter {
                letters.push(letter);
            }

            let next = node.next(direction);

            index = next.0;
            direction = next.1;

            steps += node.get_length();
        }

        (letters, steps)
    }
}
