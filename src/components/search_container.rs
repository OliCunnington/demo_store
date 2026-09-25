use leptops::prelude::*;

#[component]
pub fn SearchContainer() -> impl IntoView {
    view!{
        <div class="search_container">
            <p>"Placeholder"<p>
        <div>
    }
}

// should probably take component as arg
//  populate list... data and comp?
//  index or something for searching??

// border around children
// labeled columns
// decorated rows