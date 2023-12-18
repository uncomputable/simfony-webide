use std::fmt;
use std::sync::Arc;

use simplicity::node::Inner;

use crate::util::Expression;
use crate::value::{Bytes, ExtValue};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub expression: Expression,
    pub input: Arc<ExtValue>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]({})", self.expression, self.input)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ErrorKind {
    AssertionFailed,
    FailNode,
    JetsNotSupported,
    ExpectedProduct,
    ExpectedSumInFirstComponent,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::AssertionFailed => f.write_str("Assertion failed"),
            ErrorKind::FailNode => f.write_str("A fail node was reached"),
            ErrorKind::JetsNotSupported => f.write_str("Jets are currently not supported"),
            ErrorKind::ExpectedProduct => f.write_str("Expected a product value as input"),
            ErrorKind::ExpectedSumInFirstComponent => {
                f.write_str("Expected a sum value in the first component of the input")
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
        write!(f, "Evaluation failed: {}\n{}", self.kind, self.state)
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
    ExecuteComp(Expression),
    ExecuteDisconnect(Expression),
    MakeLeft,
    MakeRight,
    MakeProduct,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Runner {
    input: Vec<Task>,
    output: Vec<Arc<ExtValue>>,
}

impl Runner {
    pub fn for_program(program: Expression) -> Self {
        Self::for_expression(program, ExtValue::unit())
    }

    fn for_expression(expression: Expression, input: Arc<ExtValue>) -> Self {
        let initial_state = State { expression, input };
        Self {
            input: vec![Task::Execute(initial_state)],
            output: vec![],
        }
    }

    pub fn run(&mut self) -> Result<Arc<ExtValue>, Error> {
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
                    let (b, c) = prod_b_c.split_product().unwrap();
                    self.output.push(b);
                    let state = State {
                        expression: t,
                        input: c,
                    };
                    self.execute_state(state)?;
                }
                Task::MakeLeft => {
                    let a = self.output.pop().unwrap();
                    self.output.push(ExtValue::left(a));
                }
                Task::MakeRight => {
                    let a = self.output.pop().unwrap();
                    self.output.push(ExtValue::right(a));
                }
                Task::MakeProduct => {
                    let b = self.output.pop().unwrap();
                    let a = self.output.pop().unwrap();
                    self.output.push(ExtValue::product(a, b));
                }
            }
        }

        debug_assert!(self.output.len() == 1);
        let a = self.output.pop().unwrap();
        Ok(a)
    }

    fn execute_state(&mut self, state: State) -> Result<(), Error> {
        let inner = state.expression.0.inner();
        match inner {
            Inner::Iden => {
                self.output.push(state.input);
            }
            Inner::Unit => {
                self.output.push(ExtValue::unit());
            }
            Inner::InjL(t) => {
                self.input.push(Task::MakeLeft);
                let t_state = State {
                    expression: Expression(t.clone()),
                    input: state.input,
                };
                self.input.push(Task::Execute(t_state));
            }
            Inner::InjR(t) => {
                self.input.push(Task::MakeRight);
                let t_state = State {
                    expression: Expression(t.clone()),
                    input: state.input,
                };
                self.input.push(Task::Execute(t_state));
            }
            Inner::Take(t) => {
                let (a, _) = state
                    .input
                    .split_product()
                    .ok_or_else(|| Error::new(ErrorKind::ExpectedProduct, state.clone()))?;
                let t_state = State {
                    expression: Expression(t.clone()),
                    input: a,
                };
                self.input.push(Task::Execute(t_state));
            }
            Inner::Drop(t) => {
                let (_, b) = state
                    .input
                    .split_product()
                    .ok_or_else(|| Error::new(ErrorKind::ExpectedProduct, state.clone()))?;
                let t_state = State {
                    expression: Expression(t.clone()),
                    input: b,
                };
                self.input.push(Task::Execute(t_state));
            }
            Inner::Comp(s, t) => {
                self.input.push(Task::ExecuteComp(Expression(t.clone())));
                let s_state = State {
                    expression: Expression(s.clone()),
                    input: state.input,
                };
                self.input.push(Task::Execute(s_state));
            }
            Inner::Pair(s, t) => {
                self.input.push(Task::MakeProduct);
                let t_state = State {
                    expression: Expression(t.clone()),
                    input: state.input.clone(),
                };
                self.input.push(Task::Execute(t_state));
                let s_state = State {
                    expression: Expression(s.clone()),
                    input: state.input,
                };
                self.input.push(Task::Execute(s_state));
            }
            Inner::Case(..) | Inner::AssertL(..) | Inner::AssertR(..) => {
                let (sum_a_b, c) = state
                    .input
                    .split_product()
                    .ok_or_else(|| Error::new(ErrorKind::ExpectedProduct, state.clone()))?;

                if let Some(a) = sum_a_b.split_left() {
                    match inner {
                        Inner::Case(s, _) | Inner::AssertL(s, _) => {
                            let s_state = State {
                                expression: Expression(s.clone()),
                                input: ExtValue::product(a.clone(), c),
                            };
                            self.input.push(Task::Execute(s_state));
                        }
                        Inner::AssertR(_, _) => {
                            return Err(Error::new(ErrorKind::AssertionFailed, state.clone()));
                        }
                        _ => unreachable!("Covered by outer match statement"),
                    }
                } else if let Some(b) = sum_a_b.split_right() {
                    match inner {
                        Inner::Case(_, t) | Inner::AssertR(_, t) => {
                            let t_state = State {
                                expression: Expression(t.clone()),
                                input: ExtValue::product(b.clone(), c),
                            };
                            self.input.push(Task::Execute(t_state));
                        }
                        Inner::AssertL(_, _) => {
                            return Err(Error::new(ErrorKind::AssertionFailed, state.clone()));
                        }
                        _ => unreachable!("Covered by outer match statement"),
                    }
                } else {
                    return Err(Error::new(
                        ErrorKind::ExpectedSumInFirstComponent,
                        state.clone(),
                    ));
                }
            }
            Inner::Disconnect(s, t) => {
                self.input.push(Task::MakeProduct);
                self.input
                    .push(Task::ExecuteDisconnect(Expression(t.clone())));

                let t_cmr = ExtValue::bytes(Bytes::from_slice(t.cmr()));
                let s_state = State {
                    expression: Expression(s.clone()),
                    input: ExtValue::product(t_cmr, state.input),
                };
                self.input.push(Task::Execute(s_state));
            }
            Inner::Witness(value) => self.output.push(Arc::new(ExtValue::from(value.as_ref()))),
            Inner::Fail(_) => {
                return Err(Error::new(ErrorKind::FailNode, state));
            }
            Inner::Jet(_) => {
                return Err(Error::new(ErrorKind::JetsNotSupported, state));
            }
            Inner::Word(value) => self.output.push(Arc::new(ExtValue::from(value.as_ref()))),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{examples, util};

    #[test]
    fn test() {
        for (name, human) in examples::NAMED_PROGRAMS {
            let program = Expression(util::program_from_string(human).unwrap());
            let mut runner = Runner::for_program(program);
            let ret = runner.run();

            if name.contains("failure") {
                assert!(ret.is_err());
            } else {
                assert!(ret.is_ok());
            }
        }
    }
}
