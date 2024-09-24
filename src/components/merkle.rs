use std::sync::Arc;

use js_sys::{Array, Object};
use leptos::*;
use simfony::simplicity;
use simplicity::dag::DagLike;
use simplicity::dag::NoSharing;
use simplicity::node;
use wasm_bindgen::prelude::*;

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
                <div class="merkle_graph" class:hidden=move || !graph_toggle.get() >
                    <div id="merkle_graph_holder"></div>

                    <div class="graph-button-holder">
                        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" class="graph-button" on:click=move |_| manualZoom("zoom_in")>
                            <line x1="5" y1="50" x2="95" y2="50" />
                            <line x1="50" y1="5" x2="50" y2="95" />
                        </svg>

                        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" class="graph-button" on:click=move |_| manualZoom("zoom_out")>
                            <line x1="5" y1="50" x2="95" y2="50" />
                        </svg>

                        <svg viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg" class="graph-button" on:click=move |_| manualZoom("zoom_reset")>
                            <path d="M64,256H34A222,222,0,0,1,430,118.15V85h30V190H355V160h67.27A192.21,192.21,0,0,0,256,64C150.13,64,64,150.13,64,256Zm384,0c0,105.87-86.13,192-192,192A192.21,192.21,0,0,1,89.73,352H157V322H52V427H82V393.85A222,222,0,0,0,478,256Z"/>
                        </svg>
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
    fn manualZoom(mode: &str);
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
