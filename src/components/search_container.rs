use leptops::prelude::*;

#[component]
pub fn SearchContainer() -> impl IntoView {
    view!{
        <div class="search_container">
            <p>"Placeholder"<p>
            <label for="searchBox">
                <img src="/icons/search-magnifying-glass-svgrepo-com.svg" alt="Search" width="24" height="24"/>
                <input type="text" id="searchBox"/>
            </label>
        <div>
    }
}

// should probably take component as arg
//  populate list... data and comp?
//  index or something for searching??

// border around children
// labeled columns
// decorated rows