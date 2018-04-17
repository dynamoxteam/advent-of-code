use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Sequence {
    programs: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Template {
    direct: Vec<u8>,
    indirect: Vec<u8>,
}

impl Sequence {
    pub fn identity(len: u8) -> Sequence {
        Sequence {
            programs: (0..len).collect(),
        }
    }

    pub fn len(&self) -> u8 {
        self.programs.len() as u8
    }
}

impl Template {
    pub fn identity(len: u8) -> Template {
        let direct = (0..len).collect();
        let indirect = (0..len).collect();

        Template { direct, indirect }
    }

    pub fn parse(instructions: &str, len: u8) -> Template {
        let mut direct: Vec<u8> = (0..len).collect();
        let mut indirect: Vec<u8> = (0..len).collect();

        for mov in instructions.trim().split(',') {
            let (mov_type, params) = mov.split_at(1);

            match mov_type {
                "s" => spin(&mut indirect, params),
                "x" => exchange(&mut indirect, params),
                "p" => partner(&mut direct, params),
                _ => (),
            };
        }

        Template { direct, indirect }
    }

    pub fn len(&self) -> u8 {
        self.direct.len() as u8
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for p in self.programs.iter() {
            write!(f, "{}", ('a' as u8 + *p) as char)?;
        }

        Ok(())
    }
}

impl<'a> Add<&'a Template> for &'a Sequence {
    type Output = Sequence;

    fn add(self, rhs: &'a Template) -> Sequence {
        let mut programs = self.programs.clone();

        apply_indirect(&self.programs, &mut programs, &rhs.indirect);
        apply_direct(&mut programs, &rhs.direct);

        Sequence { programs }
    }
}

impl<'a> Add<Template> for &'a Sequence {
    type Output = Sequence;

    fn add(self, rhs: Template) -> Sequence {
        self + &rhs
    }
}

impl<'a> Add<&'a Template> for Sequence {
    type Output = Sequence;

    fn add(self, rhs: &'a Template) -> Sequence {
        &self + rhs
    }
}

impl Add<Template> for Sequence {
    type Output = Sequence;

    fn add(self, rhs: Template) -> Sequence {
        &self + &rhs
    }
}

impl<'a> AddAssign<&'a Template> for Sequence {
    fn add_assign(&mut self, rhs: &'a Template) {
        *self = &*self + rhs;
    }
}

impl AddAssign<Template> for Sequence {
    fn add_assign(&mut self, rhs: Template) {
        *self += &rhs;
    }
}

impl<'a> Add<&'a Template> for &'a Template {
    type Output = Template;

    fn add(self, rhs: &'a Template) -> Template {
        let mut direct = self.direct.clone();
        let mut indirect = self.indirect.clone();

        apply_indirect(&self.indirect, &mut indirect, &rhs.indirect);
        apply_direct(&mut direct, &rhs.direct);

        Template { direct, indirect }
    }
}

impl<'a> Add<Template> for &'a Template {
    type Output = Template;

    fn add(self, rhs: Template) -> Template {
        self + &rhs
    }
}

impl<'a> Add<&'a Template> for Template {
    type Output = Template;

    fn add(self, rhs: &'a Template) -> Template {
        &self + rhs
    }
}

impl Add<Template> for Template {
    type Output = Template;

    fn add(self, rhs: Template) -> Template {
        &self + &rhs
    }
}

impl<'a> AddAssign<&'a Template> for Template {
    fn add_assign(&mut self, rhs: &'a Template) {
        *self = &*self + rhs;
    }
}

impl AddAssign<Template> for Template {
    fn add_assign(&mut self, rhs: Template) {
        *self += &rhs;
    }
}

impl<'a> Mul<usize> for &'a Template {
    type Output = Template;

    fn mul(self, rhs: usize) -> Template {
        let mut output = Template::identity(self.len());
        let mut template = self.clone();

        if rhs & 1 != 0 {
            output += &template;
        }

        let mut r = rhs >> 1;

        while r != 0 {
            template += template.clone();

            if r & 1 != 0 {
                output += &template;
            }

            r >>= 1;
        }

        output
    }
}

impl Mul<usize> for Template {
    type Output = Template;

    fn mul(self, rhs: usize) -> Template {
        &self * rhs
    }
}

impl<'a> Mul<&'a Template> for usize {
    type Output = Template;

    fn mul(self, rhs: &'a Template) -> Template {
        rhs * self
    }
}

impl Mul<Template> for usize {
    type Output = Template;

    fn mul(self, rhs: Template) -> Template {
        self * &rhs
    }
}

fn spin(programs: &mut [u8], params: &str) {
    let steps = params.parse::<usize>().unwrap();

    programs.rotate_right(steps);
}

fn exchange(programs: &mut [u8], params: &str) {
    let mut indexes = params.split('/').map(|n| n.parse::<usize>().unwrap());
    let indexes = (indexes.next().unwrap(), indexes.next().unwrap());

    programs.swap(indexes.0, indexes.1);
}

fn partner(programs: &mut [u8], params: &str) {
    let indexes = {
        let mut indexes = params
            .split('/')
            .map(|n| char::from_str(n).unwrap() as u8 - 'a' as u8)
            .map(|n| programs.iter().position(|&m| m == n).unwrap());

        (indexes.next().unwrap(), indexes.next().unwrap())
    };

    programs.swap(indexes.0, indexes.1);
}

fn apply_direct(programs: &mut Vec<u8>, template: &Vec<u8>) {
    for i in 0..programs.len() {
        programs[i] = template[programs[i] as usize];
    }
}

fn apply_indirect(input: &Vec<u8>, output: &mut Vec<u8>, template: &Vec<u8>) {
    for i in 0..template.len() {
        output[i] = input[template[i] as usize];
    }
}
