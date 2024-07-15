use std::sync::Arc;

use js_sys::{Array, Object};
use leptos::*;
use simplicity::dag::DagLike;
use simplicity::node;
use wasm_bindgen::prelude::*;

use crate::simplicity;
use crate::simplicity::dag::NoSharing;
use crate::util::{DisplayInner, Expression};

#[component]
pub fn MerkleExplorer(
    run_result: ReadSignal<Option<Result<String, String>>>,
    graph_toggle: ReadSignal<bool>,
    set_graph_toggle: WriteSignal<bool>,
) -> impl IntoView {
    move || match run_result.get() {
        Some(Ok(_)) => view! {
            <div id="merkle-container" class="analysis">
                <div class="flex analysis-header">
                    <div
                        on:click=move |_| set_graph_toggle.set(!graph_toggle.get())
                        class="graph-toggle-holder"
                    >
                        <h2 class="analysis-title">Merkle Explorer</h2>

                        <svg width="46" height="24" viewBox="0 0 46 24" fill="none" xmlns="http://www.w3.org/2000/svg"
                            id="graph-toggle-icon"
                            class:toggle-on=move || graph_toggle.get()
                        >
                            <rect x="0.5" y="0.5" width="45" height="22.7931" rx="11.3966" />
                            <circle cx="11.897" cy="11.8965" r="8.72414" />
                        </svg>
                    </div>
                </div>
                <div class:hidden=move || !graph_toggle.get() >
                    <div id="merkle_graph_holder">
                    </div>
                </div>
            </div>
        },
        _ => view! {
            <div></div>
        },
    }
}

#[wasm_bindgen(module = "/src/assets/js/merkle_graph_d3.js")]
extern "C" {
    fn load_merkle_graph_js(dat: JsValue);
}

fn marshal_merkle_data<M: node::Marker>(expression: &node::Node<M>) -> JsValue {
    let mut output = vec![];
    for data in expression.post_order_iter::<NoSharing>() {
        let text = JsValue::from(DisplayInner::from(data.node).to_string());
        let children = Array::new();
        if data.left_index.is_some() {
            children.push(&output.pop().unwrap());
        }
        if data.right_index.is_some() {
            children.push(&output.pop().unwrap());
        }
        let node_obj = Object::new();
        js_sys::Reflect::set(&node_obj, &JsValue::from_str("text"), &text).unwrap();
        js_sys::Reflect::set(&node_obj, &JsValue::from_str("children"), &children).unwrap();

        output.push(JsValue::from(node_obj))
    }
    debug_assert!(output.len() == 1);
    output.pop().unwrap()
}

pub fn reload_graph(expression: Arc<Expression>) {
    let data = marshal_merkle_data(&expression);
    load_merkle_graph_js(data);
}
