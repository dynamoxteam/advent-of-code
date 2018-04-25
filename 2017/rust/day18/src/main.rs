extern crate common;
extern crate regex;

use std::char::ParseCharError;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;
use std::thread;
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
    Rcv(Register),
    Jgz(Value, Value),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Program {
    instructions: Vec<Instruction>,
}

enum Message {
    Value(isize),
    Sending(usize),
    Receiving(usize),
    Abort,
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
        let instruction_regex =
            Regex::new(r"(?m)^(?P<instruction>[[:alpha:]]+) +(?P<params>.*)$").unwrap();

        let mut instructions = Vec::<Instruction>::new();

        for instruction_cap in instruction_regex.captures_iter(input) {
            let instruction = instruction_cap.name("instruction").unwrap().as_str();

            let params: Vec<&str> = instruction_cap
                .name("params")
                .unwrap()
                .as_str()
                .trim()
                .split(' ')
                .collect();

            instructions.push(match instruction {
                "snd" => Instruction::Snd(params[0].parse::<Value>().unwrap()),
                "set" => Instruction::Set(
                    params[0].parse::<Register>().unwrap(),
                    params[1].parse::<Value>().unwrap(),
                ),
                "add" => Instruction::Add(
                    params[0].parse::<Register>().unwrap(),
                    params[1].parse::<Value>().unwrap(),
                ),
                "mul" => Instruction::Mul(
                    params[0].parse::<Register>().unwrap(),
                    params[1].parse::<Value>().unwrap(),
                ),
                "mod" => Instruction::Mod(
                    params[0].parse::<Register>().unwrap(),
                    params[1].parse::<Value>().unwrap(),
                ),
                "rcv" => Instruction::Rcv(params[0].parse::<Register>().unwrap()),
                "jgz" => Instruction::Jgz(
                    params[0].parse::<Value>().unwrap(),
                    params[1].parse::<Value>().unwrap(),
                ),
                _ => panic!("Invalid instruction"),
            });
        }

        Program { instructions }
    }

    fn execute(
        &self,
        id: usize,
        control: Sender<Message>,
        tx: Sender<Message>,
        rx: Receiver<Message>,
    ) -> Result<(), TryRecvError> {
        let mut registers = HashMap::new();
        let mut pc = 0;

        registers.insert('p', id as isize);

        while let Some(instruction) = self.instructions.get(pc as usize) {
            let mut pc_inc = 1;

            match *instruction {
                Instruction::Snd(ref val) => {
                    let msg = Message::Value(Program::read_value(val, &registers));

                    control.send(Message::Sending(id)).unwrap();
                    tx.send(msg).unwrap();
                }
                Instruction::Set(ref reg, ref val) => {
                    Program::update_value(reg, val, &mut registers, |_, y| y);
                }
                Instruction::Add(ref reg, ref val) => {
                    Program::update_value(reg, val, &mut registers, |x, y| x + y);
                }
                Instruction::Mul(ref reg, ref val) => {
                    Program::update_value(reg, val, &mut registers, |x, y| x * y);
                }
                Instruction::Mod(ref reg, ref val) => {
                    Program::update_value(reg, val, &mut registers, |x, y| x % y);
                }
                Instruction::Rcv(ref reg) => {
                    control.send(Message::Receiving(id)).unwrap();

                    let msg = rx.recv()?;

                    match msg {
                        Message::Value(n) => {
                            let val = Value::Num(n);

                            Program::update_value(reg, &val, &mut registers, |_, y| y);
                        }
                        Message::Abort => break,
                        _ => (),
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

        Ok(())
    }

    fn read_value(value: &Value, map: &HashMap<char, isize>) -> isize {
        match *value {
            Value::Num(i) => i,
            Value::Reg(ref c) => map.get(c).cloned().unwrap_or(0),
        }
    }

    fn update_value<F>(register: &Register, value: &Value, map: &mut HashMap<char, isize>, op: F)
    where
        F: Fn(isize, isize) -> isize,
    {
        let new_value = op(
            map.get(&register.0).cloned().unwrap_or(0),
            Program::read_value(value, map),
        );

        map.insert(register.0, new_value);
    }

    pub fn run(self) -> [usize; 2] {
        let program = Arc::new(self);
        let (prg0, prg1) = (program.clone(), program.clone());

        let (ctrl_tx, ctrl_rx) = mpsc::channel();
        let (prg0_tx, prg0_rx) = mpsc::channel();
        let (prg1_tx, prg1_rx) = mpsc::channel();

        let (ctrl0, tx0, rx0) = (ctrl_tx.clone(), prg1_tx.clone(), prg0_rx);
        let (ctrl1, tx1, rx1) = (ctrl_tx.clone(), prg0_tx.clone(), prg1_rx);

        let threads = (
            thread::spawn(move || prg0.execute(0, ctrl0, tx0, rx0)),
            thread::spawn(move || prg1.execute(1, ctrl1, tx1, rx1)),
        );

        let mut send_count = [0, 0];
        let mut state = 2;

        while state != 0 {
            let msg = ctrl_rx.recv().unwrap();

            match msg {
                Message::Sending(id) => {
                    state += 1;
                    send_count[id] += 1;
                }
                Message::Receiving(_) => {
                    state -= 1;
                }
                _ => (),
            }
        }

        prg0_tx.send(Message::Abort).unwrap();
        prg1_tx.send(Message::Abort).unwrap();

        threads.0.join().unwrap().unwrap();
        threads.1.join().unwrap().unwrap();

        send_count
    }
}

fn main() {
    let input = common::load_file_input("day18");
    let program = Program::load(input.as_str());

    println!("Program 1 sent: {}", program.run()[1]);
}

#[test]
fn test() {
    let program = Program::load(
        "snd 1\n\
         snd 2\n\
         snd p\n\
         rcv a\n\
         rcv b\n\
         rcv c\n\
         rcv d\n",
    );

    assert_eq!(program.run(), [3, 3]);
}
