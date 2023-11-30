use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use simplicity::jet::Jet;
use simplicity::node::Inner;
use simplicity::RedeemNode;

use crate::{exec, util};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Instruction {
    NewFrame(usize),
    MoveFrame,
    DropFrame,
    Write(bool),
    Skip(usize),
    Copy(usize),
    Fwd(usize),
    Bwd(usize),
    WriteString(Vec<bool>),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::NewFrame(bit_len) => write!(f, "newFrame({bit_len})"),
            Instruction::MoveFrame => write!(f, "moveFrame"),
            Instruction::DropFrame => write!(f, "dropFrame"),
            Instruction::Write(bit) => write!(f, "write({bit})"),
            Instruction::Skip(bit_len) => write!(f, "skip({bit_len})"),
            Instruction::Copy(bit_len) => write!(f, "copy({bit_len})"),
            Instruction::Fwd(bit_len) => write!(f, "fwd({bit_len})"),
            Instruction::Bwd(bit_len) => write!(f, "bwd({bit_len})"),
            Instruction::WriteString(bitstring) => {
                write!(f, "writeString({})", util::fmt_bitstring(bitstring))
            }
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "moveFrame" => return Ok(Instruction::MoveFrame),
            "dropFrame" => return Ok(Instruction::DropFrame),
            _ => {}
        }

        let parts: Vec<&str> = s.split(&['(', ')'][..]).collect();
        if parts.len() < 2 {
            return Err(format!("Malformed instruction: {}", s));
        }

        match parts[0] {
            "newFrame" => parts[1]
                .parse::<usize>()
                .map(Instruction::NewFrame)
                .map_err(|e| e.to_string()),
            "write" => parts[1]
                .parse::<bool>()
                .map(Instruction::Write)
                .map_err(|e| e.to_string()),
            "skip" => parts[1]
                .parse::<usize>()
                .map(Instruction::Skip)
                .map_err(|e| e.to_string()),
            "copy" => parts[1]
                .parse::<usize>()
                .map(Instruction::Copy)
                .map_err(|e| e.to_string()),
            "fwd" => parts[1]
                .parse::<usize>()
                .map(Instruction::Fwd)
                .map_err(|e| e.to_string()),
            "bwd" => parts[1]
                .parse::<usize>()
                .map(Instruction::Bwd)
                .map_err(|e| e.to_string()),
            "writeString" => util::parse_bitstring(parts[1]).map(Instruction::WriteString),
            _ => Err(format!("Unknown instruction: {}", parts[0])),
        }
    }
}

impl Instruction {
    #[allow(clippy::unit_arg)]
    pub fn execute(&self, mac: &mut exec::BitMachine) -> Result<(), exec::Error> {
        match *self {
            Instruction::NewFrame(bit_len) => Ok(mac.new_frame(bit_len)),
            Instruction::MoveFrame => mac.move_frame(),
            Instruction::DropFrame => Ok(mac.drop_frame()),
            Instruction::Write(bit) => mac.write(bit),
            Instruction::Skip(bit_len) => mac.skip(bit_len),
            Instruction::Copy(bit_len) => mac.copy(bit_len),
            Instruction::Fwd(bit_len) => mac.fwd(bit_len),
            Instruction::Bwd(bit_len) => mac.bwd(bit_len),
            Instruction::WriteString(ref bitstring) => mac.write_bitstring(bitstring),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Task<J: Jet> {
    Run(Instruction),
    TcoOff(Arc<RedeemNode<J>>),
    TcoOn(Arc<RedeemNode<J>>),
}

impl<J: Jet> fmt::Display for Task<J> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Task::Run(instruction) => write!(f, "{instruction}"),
            Task::TcoOff(node) => write!(f, "⟪{}⟫", node.inner()),
            Task::TcoOn(node) => write!(f, "⟪{}⟫tco", node.inner()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Runner<J: Jet> {
    stack: Vec<Task<J>>,
    mac: exec::BitMachine,
    optimization: bool,
}

impl<J: Jet> Runner<J> {
    pub fn for_program(program: Arc<RedeemNode<J>>, optimization: bool) -> Self {
        Self {
            stack: vec![Task::TcoOff(program)],
            mac: exec::BitMachine::for_program(),
            optimization,
        }
    }

    pub fn get_stack(&self) -> &[Task<J>] {
        &self.stack
    }

    pub fn get_mac(&self) -> &exec::BitMachine {
        &self.mac
    }

    pub fn next(&mut self) -> Result<Option<Instruction>, exec::Error> {
        while let Some(top) = self.stack.pop() {
            match top {
                Task::Run(x) => {
                    x.execute(&mut self.mac)?;
                    return Ok(Some(x));
                }
                Task::TcoOff(node) => {
                    self.tco_off(node)?;
                }
                Task::TcoOn(node) => {
                    self.tco_on(node)?;
                }
            }
        }

        Ok(None)
    }

    fn tco_off(&mut self, node: Arc<RedeemNode<J>>) -> Result<(), exec::Error> {
        let stack = &mut self.stack;
        let inner = node.inner().clone();
        match inner {
            Inner::Unit => {
                // nop; continue with next instruction
            }
            Inner::Iden => {
                let size_a = node.arrow().source.bit_width();
                stack.push(Task::Run(Instruction::Copy(size_a)));
            }
            Inner::InjL(left) => {
                let (b, _c) = node.arrow().target.split_sum().unwrap();
                let padl_b_c = node.arrow().target.bit_width() - b.bit_width() - 1;
                stack.push(Task::TcoOff(left));
                stack.push(Task::Run(Instruction::Skip(padl_b_c)));
                stack.push(Task::Run(Instruction::Write(false)));
            }
            Inner::InjR(left) => {
                let (_b, c) = node.arrow().target.split_sum().unwrap();
                let padr_b_c = node.arrow().target.bit_width() - c.bit_width() - 1;
                stack.push(Task::TcoOff(left));
                stack.push(Task::Run(Instruction::Skip(padr_b_c)));
                stack.push(Task::Run(Instruction::Write(true)));
            }
            Inner::Take(left) => {
                stack.push(Task::TcoOff(left));
            }
            Inner::Drop(left) => {
                let size_a = node.arrow().source.split_product().unwrap().0.bit_width();
                stack.push(Task::Run(Instruction::Bwd(size_a)));
                stack.push(Task::TcoOff(left));
                stack.push(Task::Run(Instruction::Fwd(size_a)));
            }
            Inner::Comp(left, right) => {
                let size_b = left.arrow().target.bit_width();
                if !self.optimization {
                    stack.push(Task::Run(Instruction::DropFrame));
                    stack.push(Task::TcoOff(right));
                } else {
                    stack.push(Task::TcoOn(right));
                }
                stack.push(Task::Run(Instruction::MoveFrame));
                stack.push(Task::TcoOff(left));
                stack.push(Task::Run(Instruction::NewFrame(size_b)));
            }
            Inner::Pair(left, right) => {
                stack.push(Task::TcoOff(right));
                stack.push(Task::TcoOff(left));
            }
            Inner::Case(..) | Inner::AssertL(..) | Inner::AssertR(..) => {
                let choice_bit = self.mac.peek()?;
                let (sum_a_b, _c) = node.arrow().source.split_product().unwrap();
                let (a, b) = sum_a_b.split_sum().unwrap();

                match (inner, choice_bit) {
                    (Inner::Case(left, _) | Inner::AssertL(left, _), false) => {
                        let padl_a_b = sum_a_b.bit_width() - a.bit_width() - 1;
                        stack.push(Task::Run(Instruction::Bwd(padl_a_b + 1)));
                        stack.push(Task::TcoOff(left));
                        stack.push(Task::Run(Instruction::Fwd(padl_a_b + 1)));
                    }
                    (Inner::Case(_, right) | Inner::AssertR(_, right), true) => {
                        let padr_a_b = sum_a_b.bit_width() - b.bit_width() - 1;
                        stack.push(Task::Run(Instruction::Bwd(padr_a_b + 1)));
                        stack.push(Task::TcoOff(right));
                        stack.push(Task::Run(Instruction::Fwd(padr_a_b + 1)));
                    }
                    (Inner::AssertL(_, right_cmr), true) => {
                        return Err(exec::Error::PrunedBranch(right_cmr));
                    }
                    (Inner::AssertR(left_cmr, _), false) => {
                        return Err(exec::Error::PrunedBranch(left_cmr));
                    }
                    _ => unreachable!("Covered by outer match statement"),
                }
            }
            Inner::Fail(entropy) => {
                return Err(exec::Error::FailNode(entropy));
            }
            Inner::Disconnect(left, right) => {
                let size_prod_256_a = left.arrow().source.bit_width();
                let size_a = size_prod_256_a - 256;
                let size_prod_b_c = left.arrow().target.bit_width();
                let size_b = size_prod_b_c - right.arrow().source.bit_width();
                let right_cmr = util::bytes_to_bitstring(right.cmr());

                if !self.optimization {
                    stack.push(Task::Run(Instruction::DropFrame));
                    stack.push(Task::Run(Instruction::DropFrame));
                    stack.push(Task::Run(Instruction::Bwd(size_b)));
                    stack.push(Task::TcoOff(right));
                    stack.push(Task::Run(Instruction::Fwd(size_b)));
                    stack.push(Task::Run(Instruction::Copy(size_b)));
                    stack.push(Task::Run(Instruction::MoveFrame));
                    stack.push(Task::TcoOff(left));
                } else {
                    stack.push(Task::TcoOn(right));
                    stack.push(Task::Run(Instruction::Fwd(size_b)));
                    stack.push(Task::Run(Instruction::Copy(size_b)));
                    stack.push(Task::Run(Instruction::MoveFrame));
                    stack.push(Task::TcoOn(left));
                }

                stack.push(Task::Run(Instruction::NewFrame(size_prod_b_c)));
                stack.push(Task::Run(Instruction::MoveFrame));
                stack.push(Task::Run(Instruction::Copy(size_a)));
                stack.push(Task::Run(Instruction::WriteString(right_cmr)));
                stack.push(Task::Run(Instruction::NewFrame(size_prod_256_a)));
            }
            Inner::Witness(value) | Inner::Word(value) => {
                let string = util::value_to_bitstring(&value);
                stack.push(Task::Run(Instruction::WriteString(string)));
            }
            Inner::Jet(..) => return Err(exec::Error::JetsNotSupported),
        }

        Ok(())
    }

    fn tco_on(&mut self, node: Arc<RedeemNode<J>>) -> Result<(), exec::Error> {
        let stack = &mut self.stack;
        let inner = node.inner().clone();
        match inner {
            Inner::Unit => {
                stack.push(Task::Run(Instruction::DropFrame));
            }
            Inner::Iden => {
                let size_a = node.arrow().source.bit_width();
                stack.push(Task::Run(Instruction::DropFrame));
                stack.push(Task::Run(Instruction::Copy(size_a)));
            }
            Inner::InjL(left) => {
                let (b, _c) = node.arrow().target.split_sum().unwrap();
                let padl_b_c = node.arrow().target.bit_width() - b.bit_width() - 1;
                stack.push(Task::TcoOn(left));
                stack.push(Task::Run(Instruction::Skip(padl_b_c)));
                stack.push(Task::Run(Instruction::Write(false)));
            }
            Inner::InjR(left) => {
                let (_b, c) = node.arrow().target.split_sum().unwrap();
                let padr_b_c = node.arrow().target.bit_width() - c.bit_width() - 1;
                stack.push(Task::TcoOn(left));
                stack.push(Task::Run(Instruction::Skip(padr_b_c)));
                stack.push(Task::Run(Instruction::Write(true)));
            }
            Inner::Take(left) => {
                stack.push(Task::TcoOn(left));
            }
            Inner::Drop(left) => {
                let size_a = node.arrow().source.split_product().unwrap().0.bit_width();
                stack.push(Task::TcoOn(left));
                stack.push(Task::Run(Instruction::Fwd(size_a)));
            }
            Inner::Comp(left, right) => {
                let size_b = left.arrow().target.bit_width();
                stack.push(Task::TcoOn(right));
                stack.push(Task::Run(Instruction::MoveFrame));
                stack.push(Task::TcoOn(left));
                stack.push(Task::Run(Instruction::NewFrame(size_b)));
            }
            Inner::Pair(left, right) => {
                stack.push(Task::TcoOn(right));
                stack.push(Task::TcoOff(left));
            }
            Inner::Case(..) | Inner::AssertL(..) | Inner::AssertR(..) => {
                let choice_bit = self.mac.peek()?;
                let (sum_a_b, _c) = node.arrow().source.split_product().unwrap();
                let (a, b) = sum_a_b.split_sum().unwrap();

                match (inner, choice_bit) {
                    (Inner::Case(left, _) | Inner::AssertL(left, _), false) => {
                        let padl_a_b = sum_a_b.bit_width() - a.bit_width() - 1;
                        stack.push(Task::TcoOn(left));
                        stack.push(Task::Run(Instruction::Fwd(padl_a_b + 1)));
                    }
                    (Inner::Case(_, right) | Inner::AssertR(_, right), true) => {
                        let padr_a_b = sum_a_b.bit_width() - b.bit_width() - 1;
                        stack.push(Task::TcoOn(right));
                        stack.push(Task::Run(Instruction::Fwd(padr_a_b + 1)));
                    }
                    (Inner::AssertL(_, right_cmr), true) => {
                        return Err(exec::Error::PrunedBranch(right_cmr));
                    }
                    (Inner::AssertR(left_cmr, _), false) => {
                        return Err(exec::Error::PrunedBranch(left_cmr));
                    }
                    _ => unreachable!("Covered by outer match statement"),
                }
            }
            Inner::Fail(entropy) => {
                return Err(exec::Error::FailNode(entropy));
            }
            Inner::Disconnect(left, right) => {
                let size_prod_256_a = left.arrow().source.bit_width();
                let size_a = size_prod_256_a - 256;
                let size_prod_b_c = left.arrow().target.bit_width();
                let size_b = size_prod_b_c - right.arrow().source.bit_width();
                let right_cmr = util::bytes_to_bitstring(right.cmr());

                stack.push(Task::TcoOn(right));
                stack.push(Task::Run(Instruction::Fwd(size_b)));
                stack.push(Task::Run(Instruction::Copy(size_b)));
                stack.push(Task::Run(Instruction::MoveFrame));
                stack.push(Task::TcoOn(left));
                stack.push(Task::Run(Instruction::NewFrame(size_prod_b_c)));
                stack.push(Task::Run(Instruction::MoveFrame));
                stack.push(Task::Run(Instruction::DropFrame));
                stack.push(Task::Run(Instruction::Copy(size_a)));
                stack.push(Task::Run(Instruction::WriteString(right_cmr)));
                stack.push(Task::Run(Instruction::NewFrame(size_prod_256_a)));
            }
            Inner::Witness(value) | Inner::Word(value) => {
                let string = util::value_to_bitstring(&value);
                stack.push(Task::Run(Instruction::WriteString(string)));
            }
            Inner::Jet(..) => return Err(exec::Error::JetsNotSupported),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn execute_string(s: &str, optimization: bool) {
        let program = util::program_from_string(s).unwrap();
        let mut runner = Runner::for_program(program, optimization);
        println!("Step 0: {}", runner.get_mac());

        for i in 1.. {
            match runner.next() {
                Ok(Some(x)) => println!("{x}"),
                Ok(None) => break,
                Err(error) => panic!("Error: {error}"),
            }

            println!("Step {i}: {}", runner.get_mac());
        }
    }

    #[test]
    fn to_string_from_string_roundtrip() {
        let instructions = [
            Instruction::NewFrame(42),
            Instruction::MoveFrame,
            Instruction::DropFrame,
            Instruction::Write(false),
            Instruction::Write(true),
            Instruction::Copy(42),
            Instruction::Skip(42),
            Instruction::Fwd(42),
            Instruction::Bwd(42),
            Instruction::WriteString(vec![true, false, true]),
        ];
        for instruction in instructions {
            let s = instruction.to_string();
            let parsed = Instruction::from_str(s.as_str()).unwrap();
            assert_eq!(instruction, parsed);
        }
    }

    #[test]
    fn execute_iden() {
        let s = "
            main := iden
        ";
        println!("Unoptimized");
        execute_string(s, false);
        println!("\nOptimized");
        execute_string(s, true);
    }

    #[test]
    fn execute_not() {
        let s = "
            not := comp (pair iden unit) (case (injr unit) (injl unit)) : 2 -> 2
            input := injl unit : 1 -> 2
            output := unit : 2 -> 1
            main := comp input (comp not output)
        ";
        println!("Unoptimized");
        execute_string(s, false);
        println!("\nOptimized");
        execute_string(s, true);
    }

    #[test]
    fn execute_word() {
        let s = "
            input := const 0xff
            output := unit
            main := comp input output
        ";
        execute_string(s, false);
    }

    #[test]
    fn execute_disconnect() {
        let s = "
            id1 := iden : 2^256 * 1 -> 2^256 * 1
            disc1 := unit
            main := comp (disconnect id1 ?hole) unit -- fixme: ?hole is named disc1
        ";
        execute_string(s, false);
    }

    #[test]
    fn execute_assert() {
        let s = "
            input := pair (const 0b0) unit
            output := assertl unit #{unit}
            main := comp input output
        ";
        execute_string(s, false);
    }

    #[test]
    #[should_panic(expected = "Jets are currently not supported")]
    fn execute_jet() {
        let s = "
            main := comp jet_version unit
        ";
        execute_string(s, false);
    }
}
