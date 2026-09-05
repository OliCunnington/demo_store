use leptos::prelude::*;

#[component]
fn ProductCard() -> impl IntoView {
    view!{
        <div class="product_card">
            <p>"Product"</p>
            <p>"Description"</p>
            <p>"- 0 +"</p>
        </div>
    }
}

#[component]
fn ProductCardLayout -> impl IntoView {
    view!{
        // 3x3 grid of prod cards...
    }
}