extern crate common;
extern crate regex;

use std::char::ParseCharError;
use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Register(char);

#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    Num(isize),
    Reg(char),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Snd(Value),
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Rcv(Value),
    Jgz(Value, Value),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Program {
    instructions: Vec<Instruction>,
}

impl FromStr for Register {
    type Err = ParseCharError;
    
    fn from_str(input: &str) -> Result<Register, Self::Err> {
        Ok(Register(input.parse::<char>()?))
    }
}

impl FromStr for Value {
    type Err = ParseCharError;
    
    fn from_str(input: &str) -> Result<Value, Self::Err> {
        let result = input.parse::<isize>();

        if let Ok(r) = result {
            return Ok(Value::Num(r));
        }

        Ok(Value::Reg(input.parse::<char>()?))
    }
}

impl Program {
    pub fn load(input: &str) -> Program {
        let instruction_regex = Regex::new(r"(?m)^(?P<instruction>[[:alpha:]]+) +(?P<params>.*)\r?$").unwrap();
        let mut instructions = Vec::<Instruction>::new();

        for instruction_cap in instruction_regex.captures_iter(input) {
            let instruction = instruction_cap
                .name("instruction")
                .unwrap()
                .as_str();

            let params: Vec<&str> = instruction_cap
                .name("params")
                .unwrap()
                .as_str()
                .split(' ')
                .collect();

            instructions.push(match instruction {
                "snd" => Instruction::Snd(params[0].parse::<Value>().unwrap()),
                "set" => Instruction::Set(
                    params[0].parse::<Register>().unwrap(),
                    params[1].parse::<Value>().unwrap()
                ),
                "add" => Instruction::Add(
                    params[0].parse::<Register>().unwrap(),
                    params[1].parse::<Value>().unwrap()
                ),
                "mul" => Instruction::Mul(
                    params[0].parse::<Register>().unwrap(),
                    params[1].parse::<Value>().unwrap()
                ),
                "mod" => Instruction::Mod(
                    params[0].parse::<Register>().unwrap(),
                    params[1].parse::<Value>().unwrap()
                ),
                "rcv" => Instruction::Rcv(params[0].parse::<Value>().unwrap()),
                "jgz" => Instruction::Jgz(
                    params[0].parse::<Value>().unwrap(),
                    params[1].parse::<Value>().unwrap()
                ),
                _ => panic!("Invalid instruction")
            });
        }

        Program { instructions }
    }

    pub fn execute(&self) -> isize {
        let mut registers = HashMap::new();
        let mut pc = 0;
        let mut last_freq = 0;

        while let Some(instruction) = self.instructions.get(pc as usize) {
            let mut pc_inc = 1;

            match *instruction {
                Instruction::Snd(ref val) => {
                    last_freq = Program::read_value(val, &registers)
                }
                Instruction::Set(ref reg, ref val) => {
                    Program::update_value(reg, val, &mut registers, |_, y| y)
                }
                Instruction::Add(ref reg, ref val) => {
                    Program::update_value(reg, val, &mut registers, |x, y| x + y)
                }
                Instruction::Mul(ref reg, ref val) => {
                    Program::update_value(reg, val, &mut registers, |x, y| x * y)
                }
                Instruction::Mod(ref reg, ref val) => {
                    Program::update_value(reg, val, &mut registers, |x, y| x % y)
                }
                Instruction::Rcv(ref val) => {
                    if Program::read_value(val, &registers) != 0 {
                        return last_freq;
                    }
                }
                Instruction::Jgz(ref val, ref offset) => {
                    if Program::read_value(val, &mut registers) > 0 {
                        pc_inc = Program::read_value(offset, &mut registers);
                    }
                }
            };

            pc += pc_inc;
        }

        0
    }

    fn read_value(value: &Value, map: &HashMap<char, isize>) -> isize {
        match *value {
            Value::Num(i) => i,
            Value::Reg(ref c) => map.get(c).cloned().unwrap_or(0)
        }
    }

    fn update_value<F>(
        register: &Register,
        value: &Value,
        map: &mut HashMap<char, isize>,
        op: F,
    )
    where
        F: Fn(isize, isize) -> isize,
    {
        let new_value = op(
            map.get(&register.0).cloned().unwrap_or(0),
            Program::read_value(value, map)
        );

        map.insert(register.0, new_value);
    }
}

fn main() {
    let input = common::load_file_input("day18");
    let program = Program::load(input.as_str());

    println!("{}", program.execute());
}

#[test]
fn test() {
    let program = Program::load(
        "set a 1\n\
        add a 2\n\
        mul a a\n\
        mod a 5\n\
        snd a\n\
        set a 0\n\
        rcv a\n\
        jgz a -1\n\
        set a 1\n\
        jgz a -2"
    );

    assert_eq!(program.execute(), 4);
}
