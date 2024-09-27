use leptos::{
    component, create_rw_signal, use_context, view, IntoView, RwSignal, SignalGetUntracked,
    SignalSet, SignalUpdate,
};
use simfony::parse::ParseFromStr;
use simfony::str::WitnessName;
use simfony::value::Value;
use simfony::witness::WitnessValues;

use crate::components::apply_changes::ApplyChanges;
use crate::components::table_form::TableForm;

fn parse_insert_value(
    witness_values: &mut WitnessValues,
    name: &str,
    ty: &str,
    value: &str,
) -> Result<(), String> {
    let parsed_name = WitnessName::parse_from_str(name)
        .map_err(|error| format!("Faulty name `{name}`\n{error}"))?;
    let parsed_ty = simfony::types::ResolvedType::parse_from_str(ty)
        .map_err(|error| format!("Faulty type for `{name}`\n{error}"))?;
    let parsed_value = Value::parse_from_str(value, &parsed_ty)
        .map_err(|error| format!("Faulty value for `{name}`\n{error}"))?;
    witness_values
        .insert(parsed_name, parsed_value)
        .map_err(|error| error.to_string())?;

    Ok(())
}

#[component]
pub fn WitnessTab() -> impl IntoView {
    let witness_values =
        use_context::<RwSignal<WitnessValues>>().expect("witness values should exist in context");
    let parse_error = create_rw_signal("".to_string());
    let apply_changes = ApplyChanges::default();

    let header = ["Name".to_string(), "Type".to_string(), "Value".to_string()];
    let initial_rows = witness_values
        .get_untracked()
        .into_iter()
        .map(|(name, value)| [name.to_string(), value.ty().to_string(), value.to_string()])
        .chain(std::iter::once(std::array::from_fn(|_| String::default())))
        .collect::<Vec<[String; 3]>>();

    let submit_witness = move |rows_inputs: Vec<[String; 3]>| {
        let mut new_witness_values = WitnessValues::empty();
        for [name, ty, value] in &rows_inputs {
            if name.is_empty() {
                continue;
            }
            if let Err(error) = parse_insert_value(&mut new_witness_values, name, ty, value) {
                parse_error.set(error);
                apply_changes.set_success(false);
                return;
            }
        }
        parse_error.update(String::clear);
        witness_values.set(new_witness_values);
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
