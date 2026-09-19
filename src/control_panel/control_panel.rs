use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::params::Params;
use leptos_router::hooks::use_params_map;

#[derive(Clone)]
struct Product {
    key: String,
    name: String,
    description: String,
    price: f32,
    stock: i32
}

#[component]
fn ProductRow(p: Product) -> impl IntoView {
    
    let params = use_params_map();
    let id = move || params.read().get("id");
    let key = p.key.clone();
    let val = p.key.clone();
    // let selected = use_context::<ReadSignal<String>>();
    view!{
        <li>
            // <a href={move || 
            //     if id().unwrap_or_default() == val {
            //         val.clone() + "/.."
            //     } else {
            //         val.clone()
            //     }
            // }>
                // <div class="prod_row">
                //     <p>{p.name}</p>
                //     <p>{p.stock}</p>
                //     <p>{p.price}</p>
                //     // need buttons... 
                // </div>
            // </a>
            // <Show
            //     when=move || { id().unwrap_or_default() == p.key.clone() }
            //     fallback= || view! {}
            // >
            //     <Outlet/>
            // </Show>
            <details>
                <summary>{p.name.clone()}</summary>
                <div class="prod_details">
                // <div class="prod_left">
                    <p>{p.name}</p>
                    <p>{p.stock}</p>
                    <p>{p.price}</p>
                    <p>{p.description}</p>
                </div>
                //     <div class="prod_right">
                //         <button>"Edit"</button>
                //         <button>"Edit"</button>
                //         <button>"Edit"</button>
                //         <button>"Edit"</button>
                //     </div>
                // </div>
            </details>
        </li>
    }
}

#[component]
pub fn ProductExpanded() -> impl IntoView {
    view!{
        <p>"Placeholder"</p>
    }
}

#[component]
pub fn ProductControlView() -> impl IntoView {
    let prods = vec![
        Product {
            key: "AAA".to_string(),
            name: "Apple".to_string(),
            description: "Juicy apple".to_string(),
            price: 0.99,
            stock: 20,
        },
        Product {
            key: "BBB".to_string(),
            name: "Banana".to_string(),
            description: "delicious banana".to_string(),
            price: 1.99,
            stock: 10,
        },
        Product {
            key: "CCC".to_string(),
            name: "Carrot".to_string(),
            description: "crunchy carrot".to_string(),
            price: 0.99,
            stock: 20,
        },
        Product {
            key: "DDD".to_string(),
            name: "Phone".to_string(),
            description: "dumb phone".to_string(),
            price: 39.99,
            stock: 20,
        },
        Product {
            key: "EEE".to_string(),
            name: "Smart Phone".to_string(),
            description: "smartphone".to_string(),
            price: 139.99,
            stock: 5,
        },
        Product {
            key: "FFF".to_string(),
            name: "Shoes".to_string(),
            description: "Shoes".to_string(),
            price: 24.99,
            stock: 3,
        },
        Product {
            key: "GGG".to_string(),
            name: "Shirts".to_string(),
            description: "multipack of shirts".to_string(),
            price: 39.99,
            stock: 7,
        }
    ];

    view!{
        <ul>
            {prods.into_iter()
                .map(|pr| view! {
                    <li><ProductRow p=pr /></li>
                })
                .collect_view()
            }
        </ul>
    }
}