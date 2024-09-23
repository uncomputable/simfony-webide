use std::collections::HashMap;

use leptos::{
    component, create_rw_signal, use_context, view, IntoView, SignalGetUntracked, SignalSet,
    SignalUpdate,
};
use simfony::error::{WithFile, WithSpan};
use simfony::parse::ParseFromStr;
use simfony::str::WitnessName;
use simfony::value::Value;

use crate::components::app::WitnessWrapper;
use crate::components::apply_changes::ApplyChanges;
use crate::components::table_form::TableForm;

// FIXME: Upstream to simfony
fn value_parse_from_str(
    s: &str,
    ty: &simfony::types::ResolvedType,
) -> Result<Value, simfony::error::RichError> {
    let expr = simfony::parse::Expression::parse_from_str(s)?;
    let expr = simfony::ast::Expression::analyze_const(&expr, ty).with_file(s)?;
    Value::from_const_expr(&expr)
        .ok_or(simfony::error::Error::ExpressionUnexpectedType(ty.clone()))
        .with_span(&expr)
        .with_file(s)
}

fn parse_value(name: &str, ty: &str, value: &str) -> Result<(WitnessName, Value), String> {
    let parsed_name = WitnessName::parse_from_str(name)
        .map_err(|error| format!("Faulty name `{name}`\n{error}"))?;
    let parsed_ty = simfony::types::ResolvedType::parse_from_str(ty)
        .map_err(|error| format!("Faulty type for `{name}`\n{error}"))?;
    let parsed_value = value_parse_from_str(value, &parsed_ty)
        .map_err(|error| format!("Faulty value for `{name}`\n{error}"))?;

    Ok((parsed_name, parsed_value))
}

#[component]
pub fn WitnessTab() -> impl IntoView {
    let witness_values = use_context::<WitnessWrapper>().expect("witness should exist in context");
    let parse_error = create_rw_signal("".to_string());
    let apply_changes = ApplyChanges::default();

    let header = ["Name".to_string(), "Type".to_string(), "Value".to_string()];
    let initial_rows = witness_values
        .0
        .get_untracked()
        .into_iter()
        .map(|(name, value)| [name.to_string(), value.ty().to_string(), value.to_string()])
        .chain(std::iter::once(std::array::from_fn(|_| String::default())))
        .collect::<Vec<[String; 3]>>();

    let submit_witness = move |rows_inputs: Vec<[String; 3]>| {
        let mut new_witness_values = HashMap::new();
        for [name, ty, value] in &rows_inputs {
            if name.is_empty() {
                continue;
            }

            match parse_value(name, ty, value) {
                Ok((parsed_name, parsed_value)) => {
                    new_witness_values.insert(parsed_name, parsed_value);
                }
                Err(error) => {
                    parse_error.set(error);
                    apply_changes.set_success(false);
                    return;
                }
            }
        }
        parse_error.update(String::clear);
        witness_values.0.set(new_witness_values);
        apply_changes.set_success(true);
    };

    view! {
        <div class="witness-table">
            <TableForm
                header=header
                initial_rows=initial_rows
                apply_changes=apply_changes
                submit=submit_witness
                error=parse_error
            />
        </div>
    }
}
