use std::fmt;
use std::sync::Arc;

use simplicity::node::Inner;

use crate::jet;
use crate::jet::JetFailed;
use crate::simplicity;
use crate::util::{DisplayExpression, Expression};
use crate::value::{Bytes, ExtValue};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub expression: Arc<Expression>,
    pub input: Arc<ExtValue>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expression = DisplayExpression::from(self.expression.as_ref());
        match self.input.as_ref() {
            ExtValue::Product(..) => write!(f, "[{}]{}", expression, self.input),
            _ => write!(f, "[{}]({})", expression, self.input),
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ErrorKind {
    AssertionFailed,
    FailNode,
    JetFailed,
    ExpectedProduct,
    ExpectedSumInFirstComponent,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::AssertionFailed => f.write_str("Assertion failed"),
            ErrorKind::FailNode => f.write_str("A fail node was reached"),
            ErrorKind::JetFailed => f.write_str("Jet failed during execution"),
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
    ExecuteComp(Arc<Expression>),
    ExecuteDisconnect(Arc<Expression>),
    MakeLeft,
    MakeRight,
    MakeProduct,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Runner {
    input: Vec<Task>,
    output: Vec<Arc<ExtValue>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Output {
    /// Intermediate state
    Intermediate(State),
    /// Final output
    Final(Arc<ExtValue>),
}

impl Runner {
    pub fn for_program(program: Arc<Expression>) -> Self {
        Self::for_expression(program, ExtValue::unit())
    }

    fn for_expression(expression: Arc<Expression>, input: Arc<ExtValue>) -> Self {
        let initial_state = State { expression, input };
        Self {
            input: vec![Task::Execute(initial_state)],
            output: vec![],
        }
    }

    pub fn run(&mut self) -> Result<Arc<ExtValue>, Error> {
        loop {
            match self.step()? {
                Output::Intermediate(..) => {}
                Output::Final(a) => return Ok(a),
            }
        }
    }

    fn step(&mut self) -> Result<Output, Error> {
        while let Some(task) = self.input.pop() {
            match task {
                Task::Execute(state) => {
                    self.execute_state(state.clone())?;
                    return Ok(Output::Intermediate(state));
                }
                Task::ExecuteComp(t) => {
                    let input = self.output.pop().unwrap();
                    let state = State {
                        expression: t,
                        input,
                    };
                    self.execute_state(state.clone())?;
                    return Ok(Output::Intermediate(state));
                }
                Task::ExecuteDisconnect(t) => {
                    let prod_b_c = self.output.pop().unwrap();
                    let (b, c) = prod_b_c.to_product().unwrap();
                    self.output.push(b);
                    let state = State {
                        expression: t,
                        input: c,
                    };
                    self.execute_state(state.clone())?;
                    return Ok(Output::Intermediate(state));
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
        Ok(Output::Final(a))
    }

    fn execute_state(&mut self, state: State) -> Result<(), Error> {
        let inner = state.expression.inner();
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
                    expression: t.clone(),
                    input: state.input,
                };
                self.input.push(Task::Execute(t_state));
            }
            Inner::InjR(t) => {
                self.input.push(Task::MakeRight);
                let t_state = State {
                    expression: t.clone(),
                    input: state.input,
                };
                self.input.push(Task::Execute(t_state));
            }
            Inner::Take(t) => {
                let (a, _) = state
                    .input
                    .to_product()
                    .ok_or_else(|| Error::new(ErrorKind::ExpectedProduct, state.clone()))?;
                let t_state = State {
                    expression: t.clone(),
                    input: a,
                };
                self.input.push(Task::Execute(t_state));
            }
            Inner::Drop(t) => {
                let (_, b) = state
                    .input
                    .to_product()
                    .ok_or_else(|| Error::new(ErrorKind::ExpectedProduct, state.clone()))?;
                let t_state = State {
                    expression: t.clone(),
                    input: b,
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
                    .to_product()
                    .ok_or_else(|| Error::new(ErrorKind::ExpectedProduct, state.clone()))?;

                if let Some(a) = sum_a_b.to_left() {
                    match inner {
                        Inner::Case(s, _) | Inner::AssertL(s, _) => {
                            let s_state = State {
                                expression: s.clone(),
                                input: ExtValue::product(a.clone(), c),
                            };
                            self.input.push(Task::Execute(s_state));
                        }
                        Inner::AssertR(_, _) => {
                            return Err(Error::new(ErrorKind::AssertionFailed, state.clone()));
                        }
                        _ => unreachable!("Covered by outer match statement"),
                    }
                } else if let Some(b) = sum_a_b.to_right() {
                    match inner {
                        Inner::Case(_, t) | Inner::AssertR(_, t) => {
                            let t_state = State {
                                expression: t.clone(),
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
                self.input.push(Task::ExecuteDisconnect(t.clone()));

                let t_cmr = ExtValue::bytes(Bytes::from_bytes(t.cmr()));
                let s_state = State {
                    expression: s.clone(),
                    input: ExtValue::product(t_cmr, state.input),
                };
                self.input.push(Task::Execute(s_state));
            }
            Inner::Witness(value) => self.output.push(Arc::new(ExtValue::from(value.as_ref()))),
            Inner::Fail(_) => {
                return Err(Error::new(ErrorKind::FailNode, state));
            }
            Inner::Jet(jet) => {
                match jet::execute_jet_with_env(jet, state.input.clone(), &crate::env::dummy()) {
                    Ok(output) => {
                        self.output.push(output);
                    }
                    Err(JetFailed) => {
                        return Err(Error::new(ErrorKind::JetFailed, state));
                    }
                }
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
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test() {
        for name in examples::get_names() {
            let program_str = examples::get_program_str(name).unwrap();
            let program = util::program_from_string(program_str).unwrap();
            let mut runner = Runner::for_program(program);
            let ret = runner.run();

            if name.contains('❌') {
                assert!(ret.is_err());
            } else {
                assert!(ret.is_ok());
            }
        }
    }

    #[test]
    fn trace_program() {
        let program_str = examples::get_program_str("BIP 340 Schnorr").unwrap();
        let program = util::program_from_string(program_str).unwrap();
        let mut runner = Runner::for_program(program);
        loop {
            match runner.step().unwrap() {
                Output::Intermediate(state) => {
                    println!("{}", state);
                }
                Output::Final(value) => {
                    println!("{}", value);
                    break;
                }
            }
        }
    }
}
