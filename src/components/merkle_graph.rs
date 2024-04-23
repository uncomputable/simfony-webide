use leptos::*;
use std::sync::Arc;

use simplicity::dag::DagLike;
use crate::util::{DisplayInner, Expression};

#[component]
pub fn MerkleGraph(program: Signal<Result<Arc<Expression>, String>>) -> impl IntoView {
    view! {
        <div class="">
        {
            move || program.get().ok().map(|t| view! {
                <h2>Merkle Graph</h2>
                
                <div id="merkle_graph_holder">
                    <svg></svg>
                </div>
            })
        }
         </div>
    }
}
