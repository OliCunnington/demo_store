use leptos::prelude::*;
// use leptos_router::components::{Outlet, A};

#[derive(Clone)]
struct Product {
    key: i32,
    name: String,
    description: String,
    price: f32,
    stock: i32
}


#[component]
fn ProductCard(p: Product) -> impl IntoView {
    view!{
        <a class="prod_card" href={p.key.clone()}>
            <p>{p.name}</p>
            <p>{p.description}</p>
            <p>{p.price}</p>
            <p>"- 0 +"</p>
            <p>{p.stock}</p>
        </a>
    }
}

#[component]
pub fn ProductCardLayout() -> impl IntoView {
    let prods = vec![
        Product {
            key: 1,
            name: "Apple".to_string(),
            description: "Juicy apple".to_string(),
            price: 0.99,
            stock: 20,
        },
        Product {
            key: 2,
            name: "Banana".to_string(),
            description: "delicious banana".to_string(),
            price: 1.99,
            stock: 10,
        },
        Product {
            key: 3,
            name: "Carrot".to_string(),
            description: "crunchy carrot".to_string(),
            price: 0.99,
            stock: 20,
        },
        Product {
            key: 4,
            name: "Phone".to_string(),
            description: "dumb phone".to_string(),
            price: 39.99,
            stock: 20,
        },
        Product {
            key: 5,
            name: "Smart Phone".to_string(),
            description: "smartphone".to_string(),
            price: 139.99,
            stock: 5,
        },
        Product {
            key: 6,
            name: "Shoes".to_string(),
            description: "Shoes".to_string(),
            price: 24.99,
            stock: 3,
        },
        Product {
            key: 7,
            name: "Shirts".to_string(),
            description: "multipack of shirts".to_string(),
            price: 39.99,
            stock: 7,
        }
    ];

    view!{
        // 3x3 grid of prod cards...
        <ul class="prod_list">
            {prods.into_iter()
                .map(|pr| view! {
                    <li><ProductCard p=pr /></li>
                })
                .collect_view()
            }
        </ul>
    }
}