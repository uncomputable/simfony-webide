use std::sync::Arc;

use simfony::simplicity;
use simplicity::types::CompleteBound;
use simplicity::types::Final;
use simplicity::Value;

// TODO: Upstream to rust-simplicity
pub fn from_padded_bits<I: Iterator<Item = bool>>(it: &mut I, ty: &Final) -> Option<Value> {
    enum Task<'a> {
        ReadType(&'a Final),
        MakeLeft(Arc<Final>),
        MakeRight(Arc<Final>),
        MakeProduct,
    }

    let mut stack = vec![Task::ReadType(ty)];
    let mut output = vec![];

    while let Some(task) = stack.pop() {
        match task {
            Task::ReadType(ty) => match ty.bound() {
                CompleteBound::Unit => {
                    output.push(Value::unit());
                }
                CompleteBound::Sum(ty_l, ty_r) => {
                    if !it.next()? {
                        for _ in 0..ty_l.pad_left(ty_r) {
                            let _padding = it.next()?;
                        }
                        stack.push(Task::MakeLeft(Arc::clone(ty_r)));
                        stack.push(Task::ReadType(ty_l));
                    } else {
                        for _ in 0..ty_l.pad_right(ty_r) {
                            let _padding = it.next()?;
                        }
                        stack.push(Task::MakeRight(Arc::clone(ty_l)));
                        stack.push(Task::ReadType(ty_r));
                    }
                }
                CompleteBound::Product(ty_l, ty_r) => {
                    stack.push(Task::MakeProduct);
                    stack.push(Task::ReadType(ty_r));
                    stack.push(Task::ReadType(ty_l));
                }
            },
            Task::MakeLeft(ty_r) => {
                let val_l = output.pop().unwrap();
                output.push(Value::left(val_l, ty_r));
            }
            Task::MakeRight(ty_l) => {
                let val_r = output.pop().unwrap();
                output.push(Value::right(ty_l, val_r));
            }
            Task::MakeProduct => {
                let val_r = output.pop().unwrap();
                let val_l = output.pop().unwrap();
                output.push(Value::product(val_l, val_r));
            }
        }
    }

    debug_assert_eq!(output.len(), 1);
    output.pop()
}
