use leptos::prelude::*;

#[component]
pub fn ProductRow() -> impl IntoView {
    view!{
        <div class="prod_row">
            <p>"Placeholder"</p>
            <img src="/icons/edit-svgrepo-com.svg" alt="Edit" width="24" height="24"/>
            <img src="/icons/info-square-svgrepo-com.svg" alt="Info" width="24" height="24"/>
            <img src="/icons/more-vertical-svgrepo-com.svg" alt="More" width="24" height="24"/>
            <img src="/icons/x-square-svgrepo-com.svg" alt="Delete" width="24" height="24"/>
        </div>
    }
}

// want a line with name & basic info, action buttons on right
// view, edit, delete

// modal
//  uneditable and editable fields

// svg or https://rust-ui.com/icons??