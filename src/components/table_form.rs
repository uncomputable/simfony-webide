use crate::components::apply_changes::ApplyChanges;
use leptos::{
    component, create_node_ref, create_signal, ev, html, view, CollectView, For, IntoView, NodeRef,
    ReadSignal, Signal, SignalGet, SignalSet, SignalUpdate,
};

use crate::components::string_box::ErrorBox;

#[derive(Clone)]
struct Cell {
    node_ref: NodeRef<html::Input>,
    value: Option<String>,
}

impl Cell {
    pub fn new(value: Option<String>) -> Self {
        Self {
            node_ref: create_node_ref(),
            value,
        }
    }
}

#[derive(Clone)]
struct Row<const N: usize> {
    id: usize,
    cells: [Cell; N],
}

impl<const N: usize> Row<N> {
    pub fn new(id: usize, values: Option<[String; N]>) -> Self {
        let mut it = values.into_iter().flat_map(IntoIterator::into_iter);
        Self {
            id,
            cells: std::array::from_fn(|_| Cell::new(it.next())),
        }
    }

    pub fn input(self) -> [String; N] {
        std::array::from_fn(|i| {
            self.cells[i]
                .node_ref
                .get()
                .expect("<input> should be mounted")
                .value()
        })
    }
}

#[component]
pub fn TableForm<Submit, const N: usize>(
    header: [String; N],
    initial_rows: Vec<[String; N]>,
    apply_changes: ApplyChanges,
    submit: Submit,
    #[prop(into)] error: Signal<String>,
) -> impl IntoView
where
    Submit: Fn(Vec<[String; N]>) + 'static,
{
    let (next_row_id, set_next_row_id) = create_signal(initial_rows.len());
    let new_row = move || -> Row<N> {
        let id = next_row_id.get();
        set_next_row_id.set(id + 1);
        Row::<N>::new(id, None)
    };
    let (rows, set_rows) = create_signal(
        initial_rows
            .into_iter()
            .enumerate()
            .map(|(id, row_values)| Row::<N>::new(id, Some(row_values)))
            .collect::<Vec<Row<N>>>(),
    );
    let push_row = move |_event: ev::MouseEvent| {
        set_rows.update(|stack| stack.push(new_row()));
    };
    let pop_row = move |_event: ev::MouseEvent| {
        set_rows.update(|stack| {
            if 1 < stack.len() {
                stack.pop();
            }
        })
    };
    let submit_form = move |event: ev::SubmitEvent| {
        event.prevent_default(); // stop page from reloading
        let rows_inputs: Vec<[String; N]> = rows.get().into_iter().map(Row::input).collect();
        submit(rows_inputs)
    };

    view! {
        <div class="table-form">
            <form on:submit=submit_form>
                <table>
                    <Header header=header />
                    <Body rows=rows />
                </table>
                <ErrorBox error=error />
                <div class="table-form-buttons button-row">
                    <button
                        class="flat-button bordered"
                        type="button"
                        on:click=push_row
                    >
                        Row
                        <i class="fas fa-plus"></i>
                    </button>
                    <button
                        class="flat-button bordered"
                        type="button"
                        on:click=pop_row
                    >
                        Row
                        <i class="fas fa-minus"></i>
                    </button>
                    {apply_changes}
                </div>
            </form>
        </div>
    }
}

#[component]
fn Header<const N: usize>(header: [String; N]) -> impl IntoView {
    view! {
        <thead>
            <tr>
            {
                header.into_iter().map(|col| view! {
                    <th>{col}</th>
                }).collect_view()
            }
            </tr>
        </thead>
    }
}

#[component]
fn Body<const N: usize>(rows: ReadSignal<Vec<Row<N>>>) -> impl IntoView {
    let child_view = move |row: Row<N>| {
        view! {
            <tr>
            {
                row.cells.into_iter().map(|cell| {
                    let cell_node_ref = cell.node_ref;
                    let cell_value = cell.value.unwrap_or_default();
                    view! {
                        <td>
                            <input class="input" type="text" node_ref=cell_node_ref value=cell_value />
                        </td>
                    }
                }).collect_view()
            }
            </tr>
        }
    };

    view! {
        <tbody>
            <For
                each=move || rows.get()
                key=|row| row.id
                children=child_view
            />
        </tbody>
    }
}
