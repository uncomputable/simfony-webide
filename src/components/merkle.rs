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
pub fn MerkleGraph() -> impl IntoView {
    view! {
        <div class="">
            <h2>Merkle Graph</h2>

            <div id="merkle_graph_holder">
                <svg></svg>
            </div>
         </div>
    }
}

// load js functions
#[wasm_bindgen(module = "/src/assets/js/merkle_graph_d3.js")]
extern "C" {
    fn load_merkle_graph_js(dat: JsValue);
}

pub fn reload_graph(){
    
    #[derive(Serialize, Deserialize)]
    #[wasm_bindgen]
    struct Dat {
        text: String,
        children: Vec<Dat>
    }

    // fake data **********
    let mut dat = Dat {
        text: String::from("root node"),
        children: Vec::new()
    };

    dat.children.push(Dat {
        text: String::from("node 1"),
        children: Vec::new()
    });

    dat.children.push(Dat {
        text: String::from("node 2"),
        children: Vec::new()
    });

    dat.children[0].children.push(Dat {
        text: String::from("node 3"),
        children: Vec::new()
    });

    dat.children[0].children.push(Dat {
        text: String::from("node 4"),
        children: Vec::new()
    });
    // *********************

    let d = serde_wasm_bindgen::to_value(&dat).unwrap();

    load_merkle_graph_js(d);
}

