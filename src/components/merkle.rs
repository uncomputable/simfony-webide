use leptos::*;
use std::sync::Arc;

use simplicity::dag::DagLike;
use simplicity::jet::Elements;
use simplicity::RedeemNode;

use crate::util::DisplayInner;

#[component]
pub fn Merkle(program: Signal<Result<Arc<RedeemNode<Elements>>, String>>) -> impl IntoView {
    view! {
        <div>
        {
            move || program.get().ok().map(|t| view! {
                <h2>Merkle tree</h2>
                <p>A Simplicity program is a Merkle tree, which makes it easy to analyze.</p>
                <MerkleRec expression=t/>
            })
        }
         </div>
    }
}

#[component]
fn MerkleRec(expression: Arc<RedeemNode<Elements>>) -> impl IntoView {
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
