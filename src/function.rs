use std::fmt;
use std::sync::Arc;

use simplicity::node::Inner;
use simplicity::types::Final;
use simplicity::Value;

use crate::jet;
use crate::jet::JetFailed;
use crate::simplicity;
use crate::util::Expression;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub expression: Arc<Expression>,
    pub input: Value,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.input.as_product().is_some() {
            true => {
                write!(f, "[{}]{}", self.expression.display_expr(), self.input)
            }
            false => write!(f, "[{}]({})", self.expression.display_expr(), self.input),
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ErrorKind {
    AssertionFailed,
    FailNode,
    JetFailed,
    WrongType,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::AssertionFailed => f.write_str("Assertion failed"),
            ErrorKind::FailNode => f.write_str("Universal fail"),
            ErrorKind::JetFailed => f.write_str("Jet failed"),
            ErrorKind::WrongType => {
                f.write_str("The program is ill-typed (this should never happen)")
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    state: State,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind, self.state)
    }
}

impl Error {
    pub fn new(kind: ErrorKind, state: State) -> Self {
        Self { kind, state }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Task {
    Execute(State),
    ExecuteComp(Arc<Expression>),
    ExecuteDisconnect(Arc<Expression>),
    MakeLeft(Arc<Final>),
    MakeRight(Arc<Final>),
    MakeProduct,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Runner {
    /// Stack of tasks to run.
    input: Vec<Task>,
    /// Stack of produced outputs.
    output: Vec<Value>,
}

impl Runner {
    pub fn for_program(program: Arc<Expression>) -> Self {
        let initial_state = State {
            expression: program,
            input: Value::unit(),
        };
        Self {
            input: vec![Task::Execute(initial_state)],
            output: vec![],
        }
    }

    pub fn run(&mut self) -> Result<Value, Error> {
        while let Some(task) = self.input.pop() {
            match task {
                Task::Execute(state) => {
                    self.execute_state(state)?;
                }
                Task::ExecuteComp(t) => {
                    let input = self.output.pop().unwrap();
                    let state = State {
                        expression: t,
                        input,
                    };
                    self.execute_state(state)?;
                }
                Task::ExecuteDisconnect(t) => {
                    let prod_b_c = self.output.pop().unwrap();
                    let (b, c) = prod_b_c.as_product().unwrap();
                    self.output.push(b.shallow_clone());
                    let state = State {
                        expression: t,
                        input: c.shallow_clone(),
                    };
                    self.execute_state(state)?;
                }
                Task::MakeLeft(ty_r) => {
                    let val_l = self.output.pop().unwrap();
                    self.output.push(Value::left(val_l, ty_r));
                }
                Task::MakeRight(ty_l) => {
                    let val_r = self.output.pop().unwrap();
                    self.output.push(Value::right(ty_l, val_r));
                }
                Task::MakeProduct => {
                    let b = self.output.pop().unwrap();
                    let a = self.output.pop().unwrap();
                    self.output.push(Value::product(a, b));
                }
            }
        }

        debug_assert_eq!(self.output.len(), 1);
        Ok(self.output.pop().unwrap())
    }

    fn execute_state(&mut self, state: State) -> Result<(), Error> {
        let inner = state.expression.inner();
        match inner {
            Inner::Iden => {
                self.output.push(state.input);
            }
            Inner::Unit => {
                self.output.push(Value::unit());
            }
            Inner::InjL(t) => {
                let ty_r = state.expression.arrow().target.as_sum().unwrap().1;
                self.input.push(Task::MakeLeft(Arc::new(ty_r.clone())));
                let t_state = State {
                    expression: t.clone(),
                    input: state.input,
                };
                self.input.push(Task::Execute(t_state));
            }
            Inner::InjR(t) => {
                let ty_l = state.expression.arrow().target.as_sum().unwrap().0;
                self.input.push(Task::MakeRight(Arc::new(ty_l.clone())));
                let t_state = State {
                    expression: t.clone(),
                    input: state.input,
                };
                self.input.push(Task::Execute(t_state));
            }
            Inner::Take(t) => {
                let (a, _) = state
                    .input
                    .as_product()
                    .ok_or_else(|| Error::new(ErrorKind::WrongType, state.clone()))?;
                let t_state = State {
                    expression: t.clone(),
                    input: a.shallow_clone(),
                };
                self.input.push(Task::Execute(t_state));
            }
            Inner::Drop(t) => {
                let (_, b) = state
                    .input
                    .as_product()
                    .ok_or_else(|| Error::new(ErrorKind::WrongType, state.clone()))?;
                let t_state = State {
                    expression: t.clone(),
                    input: b.shallow_clone(),
                };
                self.input.push(Task::Execute(t_state));
            }
            Inner::Comp(s, t) => {
                self.input.push(Task::ExecuteComp(t.clone()));
                let s_state = State {
                    expression: s.clone(),
                    input: state.input,
                };
                self.input.push(Task::Execute(s_state));
            }
            Inner::Pair(s, t) => {
                self.input.push(Task::MakeProduct);
                let t_state = State {
                    expression: t.clone(),
                    input: state.input.clone(),
                };
                self.input.push(Task::Execute(t_state));
                let s_state = State {
                    expression: s.clone(),
                    input: state.input,
                };
                self.input.push(Task::Execute(s_state));
            }
            Inner::Case(..) | Inner::AssertL(..) | Inner::AssertR(..) => {
                let (sum_a_b, c) = state
                    .input
                    .as_product()
                    .ok_or_else(|| Error::new(ErrorKind::WrongType, state.clone()))?;

                if let Some(a) = sum_a_b.as_left() {
                    match inner {
                        Inner::Case(s, _) | Inner::AssertL(s, _) => {
                            let s_state = State {
                                expression: s.clone(),
                                input: Value::product(a.shallow_clone(), c.shallow_clone()),
                            };
                            self.input.push(Task::Execute(s_state));
                        }
                        Inner::AssertR(_, _) => {
                            return Err(Error::new(ErrorKind::AssertionFailed, state.clone()));
                        }
                        _ => unreachable!("Covered by outer match statement"),
                    }
                } else if let Some(b) = sum_a_b.as_right() {
                    match inner {
                        Inner::Case(_, t) | Inner::AssertR(_, t) => {
                            let t_state = State {
                                expression: t.clone(),
                                input: Value::product(b.shallow_clone(), c.shallow_clone()),
                            };
                            self.input.push(Task::Execute(t_state));
                        }
                        Inner::AssertL(_, _) => {
                            return Err(Error::new(ErrorKind::AssertionFailed, state));
                        }
                        _ => unreachable!("Covered by outer match statement"),
                    }
                } else {
                    return Err(Error::new(ErrorKind::WrongType, state));
                }
            }
            Inner::Disconnect(s, t) => {
                self.input.push(Task::MakeProduct);
                self.input.push(Task::ExecuteDisconnect(t.clone()));

                let t_cmr = Value::u256(t.cmr().to_byte_array());
                let s_state = State {
                    expression: s.clone(),
                    input: Value::product(t_cmr, state.input),
                };
                self.input.push(Task::Execute(s_state));
            }
            Inner::Witness(value) => self.output.push(value.shallow_clone()),
            Inner::Fail(_) => {
                return Err(Error::new(ErrorKind::FailNode, state));
            }
            Inner::Jet(jet) => {
                match jet::execute_jet_with_env(jet, &state.input, &simfony::dummy_env::dummy()) {
                    Ok(output) => {
                        self.output.push(output);
                    }
                    Err(JetFailed) => {
                        return Err(Error::new(ErrorKind::JetFailed, state));
                    }
                }
            }
            Inner::Word(value) => self.output.push(value.shallow_clone()),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{examples, util};

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test() {
        for name in examples::get_names() {
            // Skip tutorial lessons
            if name.contains('💡') {
                continue;
            }

            println!("{name}");
            let program_str = examples::get_program_str(name).unwrap();
            let program = util::program_from_string(program_str).unwrap();
            let mut runner = Runner::for_program(program);
            match runner.run() {
                Ok(..) if name.contains('❌') => panic!("Expected failure"),
                Ok(..) => {}
                Err(..) if name.contains('❌') => {}
                Err(error) => panic!("Unexpected error: {error}"),
            }
        }
    }
}
