use std::fmt;
use std::sync::Arc;

use either::Either;
use simfony::debug::{DebugSymbols, FallibleCall, FallibleCallName};
use simfony::simplicity;
use simfony::SatisfiedProgram;
use simplicity::node::Inner;
use simplicity::types::Final;
use simplicity::Value;

use crate::jet;
use crate::jet::JetFailed;
use crate::util::Expression;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ErrorKind {
    AssertionFailed,
    FailNode,
    JetFailed,
    WrongType,
    SimfonyCallFailed(FallibleCall),
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
            ErrorKind::SimfonyCallFailed(call) => {
                match call.name() {
                    FallibleCallName::Assert => writeln!(f, "Assertion failed: false")?,
                    FallibleCallName::Panic => writeln!(f, "Explicit panic")?,
                    FallibleCallName::Jet => writeln!(f, "Jet failed")?,
                    FallibleCallName::UnwrapLeft(val) => {
                        writeln!(f, "Called `unwrap_left()` on a `Right` value: {val}")?
                    }
                    FallibleCallName::UnwrapRight(val) => {
                        writeln!(f, "Called `unwrap_right()` on a `Left` value: {val}")?
                    }
                    FallibleCallName::Unwrap => writeln!(f, "Called `unwrap()` on a `None` value")?,
                }
                write!(f, "`{}`", call.text())
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Task {
    Execute(Arc<Expression>),
    MoveLeftCompOutput,
    MoveLeftDisconnectOutput,
    MakeLeft(Arc<Final>),
    MakeRight(Arc<Final>),
    MakeProduct,
    ResetActiveSimfonyCall,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Runner {
    /// Stack of tasks to run.
    tasks: Vec<Task>,
    /// Stack of input values.
    input: Vec<Value>,
    /// Stack of output values.
    output: Vec<Value>,
    /// Simfony debug symbols in the Simplicity target code.
    debug_symbols: DebugSymbols,
    /// Simfony call expression that is currently running.
    active_simfony_call: Option<FallibleCall>,
}

impl Runner {
    pub fn for_program(program: SatisfiedProgram) -> Self {
        Self {
            tasks: vec![Task::Execute(program.simplicity)],
            input: vec![Value::unit()],
            output: vec![],
            debug_symbols: program.debug_symbols,
            active_simfony_call: None,
        }
    }

    pub fn run(&mut self) -> Result<(), ErrorKind> {
        while let Some(task) = self.tasks.pop() {
            match task {
                Task::Execute(expression) => {
                    let input = self.input.pop().unwrap();
                    match expression.inner() {
                        Inner::Iden => self.output.push(input),
                        Inner::Unit => self.output.push(Value::unit()),
                        Inner::InjL(t) => {
                            let ty_r = expression.arrow().target.as_sum().unwrap().1;
                            self.tasks.push(Task::MakeLeft(Arc::new(ty_r.clone())));
                            self.tasks.push(Task::Execute(Arc::clone(t)));
                            self.input.push(input);
                        }
                        Inner::InjR(t) => {
                            let ty_l = expression.arrow().target.as_sum().unwrap().0;
                            self.tasks.push(Task::MakeRight(Arc::new(ty_l.clone())));
                            self.tasks.push(Task::Execute(Arc::clone(t)));
                            self.input.push(input);
                        }
                        Inner::Take(t) => {
                            let (a, _) = input.as_product().ok_or(ErrorKind::WrongType)?;
                            self.tasks.push(Task::Execute(Arc::clone(t)));
                            self.input.push(a.shallow_clone());
                        }
                        Inner::Drop(t) => {
                            let (_, b) = input.as_product().ok_or(ErrorKind::WrongType)?;
                            self.tasks.push(Task::Execute(Arc::clone(t)));
                            self.input.push(b.shallow_clone());
                        }
                        Inner::Comp(s, t) => {
                            self.tasks.push(Task::Execute(Arc::clone(t)));
                            self.tasks.push(Task::MoveLeftCompOutput);
                            self.tasks.push(Task::Execute(Arc::clone(s)));
                            self.input.push(input);
                        }
                        Inner::Pair(s, t) => {
                            self.tasks.push(Task::MakeProduct);
                            self.tasks.push(Task::Execute(Arc::clone(t)));
                            self.tasks.push(Task::Execute(Arc::clone(s)));
                            self.input.push(input.shallow_clone());
                            self.input.push(input);
                        }
                        Inner::Case(..) | Inner::AssertL(..) | Inner::AssertR(..) => {
                            let (sum_a_b, c) = input.as_product().ok_or(ErrorKind::WrongType)?;

                            if let Inner::AssertL(_, cmr) = expression.inner() {
                                if let Some(tracked_call) = self.debug_symbols.get(cmr) {
                                    match tracked_call.map_value(
                                        &simfony::value::StructuralValue::from(c.shallow_clone()),
                                    ) {
                                        Some(Either::Left(fallible_call)) => {
                                            let replaced =
                                                self.active_simfony_call.replace(fallible_call);
                                            debug_assert!(replaced.is_none());
                                            self.tasks.push(Task::ResetActiveSimfonyCall);
                                        }
                                        Some(Either::Right(_debug_value)) => {}
                                        None => {}
                                    }
                                }
                            }

                            if let Some(a) = sum_a_b.as_left() {
                                match expression.inner() {
                                    Inner::Case(s, _) | Inner::AssertL(s, _) => {
                                        self.tasks.push(Task::Execute(Arc::clone(s)));
                                        self.input.push(Value::product(
                                            a.shallow_clone(),
                                            c.shallow_clone(),
                                        ));
                                    }
                                    Inner::AssertR(_, _) => {
                                        return Err(self.error(ErrorKind::AssertionFailed))
                                    }
                                    _ => unreachable!("Covered by outer match statement"),
                                }
                            } else if let Some(b) = sum_a_b.as_right() {
                                match expression.inner() {
                                    Inner::Case(_, t) | Inner::AssertR(_, t) => {
                                        self.tasks.push(Task::Execute(Arc::clone(t)));
                                        self.input.push(Value::product(
                                            b.shallow_clone(),
                                            c.shallow_clone(),
                                        ));
                                    }
                                    Inner::AssertL(_, _) => {
                                        return Err(self.error(ErrorKind::AssertionFailed))
                                    }
                                    _ => unreachable!("Covered by outer match statement"),
                                }
                            } else {
                                return Err(ErrorKind::WrongType);
                            }
                        }
                        Inner::Disconnect(s, t) => {
                            self.tasks.push(Task::MakeProduct);
                            self.tasks.push(Task::Execute(Arc::clone(t)));
                            self.tasks.push(Task::MoveLeftDisconnectOutput);
                            self.tasks.push(Task::Execute(Arc::clone(s)));
                            let t_cmr = Value::u256(t.cmr().to_byte_array());
                            self.input.push(Value::product(t_cmr, input));
                        }
                        Inner::Witness(value) => self.output.push(value.shallow_clone()),
                        Inner::Fail(_) => return Err(self.error(ErrorKind::FailNode)),
                        Inner::Jet(jet) => match jet::execute_jet_with_env(
                            jet,
                            &input,
                            &simfony::dummy_env::dummy(),
                        ) {
                            Ok(output) => self.output.push(output),
                            Err(JetFailed) => return Err(self.error(ErrorKind::JetFailed)),
                        },
                        Inner::Word(value) => self.output.push(value.shallow_clone()),
                    }
                }
                Task::MoveLeftCompOutput => {
                    let output = self.output.pop().unwrap();
                    self.input.push(output);
                }
                Task::MoveLeftDisconnectOutput => {
                    let prod_b_c = self.output.pop().unwrap();
                    let (b, c) = prod_b_c.as_product().unwrap();
                    self.output.push(b.shallow_clone());
                    self.input.push(c.shallow_clone());
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
                Task::ResetActiveSimfonyCall => self.active_simfony_call = None,
            }
        }

        debug_assert!(self.input.is_empty());
        debug_assert_eq!(self.output.len(), 1);
        debug_assert!(self.output.pop().unwrap().is_unit());
        Ok(())
    }

    /// Try to return an error with Simfony debug information included.
    /// Otherwise, return the original error.
    fn error(&self, error: ErrorKind) -> ErrorKind {
        match &self.active_simfony_call {
            Some(call) => ErrorKind::SimfonyCallFailed(call.clone()),
            None => error,
        }
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
