use leptos::*;
use std::sync::Arc;

use simplicity::dag::DagLike;
use simplicity::jet::Elements;
use simplicity::RedeemNode;

use crate::util::DisplayInner;

#[component]
pub fn Merkle(expression: Arc<RedeemNode<Elements>>) -> impl IntoView {
    let inner = DisplayInner::from(expression.as_ref()).to_string();
    let maybe_s = expression.left_child();
    let maybe_t = expression.right_child();

    view! {
        <ul>
            <li>
                <span>{inner}</span>
                {
                    move || maybe_s.clone().map(|s| view! { <Merkle expression=s/> })
                }
                {
                    move || maybe_t.clone().map(|t| view! { <Merkle expression=t/> })
                }
            </li>
        </ul>
    }
}
