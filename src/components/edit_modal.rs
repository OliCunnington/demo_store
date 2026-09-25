use leptops::prelude::*;

#[component]
pub fn EditModal(
    dialog_ref: NodeRef<html::Dialog>
) -> impl IntoView {
    view!{
        <dialog node_ref=dialog_ref class="modal">
            <p>"Placeholder"</p>
        </dialog>
    }
}


// hmm, considering this needs to be context dependent... it should probably
// take either a view or specifically a form as an arg? 

// db connections... for write/edit