use leptos::*;
use std::sync::Arc;

use crate::simplicity;
use crate::util::{DisplayInner, Expression};
use simplicity::dag::DagLike;

use crate::wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[component]
pub fn Merkle(program: Signal<Result<Arc<Expression>, String>>) -> impl IntoView {
    view! {
        {
            move || program.get().ok().map(|t| view! {
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

pub fn reload_graph(expression: Arc<Expression>){
    #[derive(Serialize, Deserialize)]
    #[wasm_bindgen]
    struct Node {
        text: String,
        children: Vec<Node>
    }

    fn merkle_data(expression: Arc<Expression>) -> Node{
        let inner = DisplayInner::from(expression.as_ref()).to_string();
        let maybe_s = expression.left_child();
        let maybe_t = expression.right_child();

        let mut node = Node {
            text: inner,
            children: Vec::new()
        };

        match maybe_s {
            Some(x) => node.children.push(merkle_data(x)),
            None => ()
        };

        match maybe_t {
            Some(x) => node.children.push(merkle_data(x)),
            None => ()
        };
        
        return node;
    }
    
    let tree = merkle_data(expression);
    let data = serde_wasm_bindgen::to_value(&tree).unwrap();
    load_merkle_graph_js(data);
}

