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
pub fn Merkle(program: Signal<Option<Arc<Expression>>>) -> impl IntoView {
    view! {
        {
            move || program.get().map(|t| view! {
                <div class="merkle">
                    <h2>Merkle tree</h2>
                    <p>A Simplicity program is a Merkle tree, which makes it easy to analyze.</p>

                    <MerkleRec expression=t/>
                </div>
            })
        }
    }
}

#[component]
fn MerkleRec(expression: Arc<Expression>) -> impl IntoView {
    let inner = DisplayInner::from(expression.as_ref()).to_string();
    let maybe_s = expression.left_child();
    let maybe_t = expression.right_child();

    view! {
        <ul>
            <li>
                <span>{inner}</span>
                {
                    move || maybe_s.clone().map(|s| view! { <MerkleRec expression=s/> })
                }
                {
                    move || maybe_t.clone().map(|t| view! { <MerkleRec expression=t/> })
                }
            </li>
        </ul>
    }
}

#[component]
pub fn MerkleGraph(run_result: Result<String, String>) -> impl IntoView {
    match run_result {
        Ok(_) => view! {
            <div id="merkle_graph_holder"></div>
        },
        Err(_) => view! {
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
