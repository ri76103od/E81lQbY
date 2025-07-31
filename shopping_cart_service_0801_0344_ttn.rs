use warp::http::StatusCode;
# 添加错误处理
use warp::{Filter, Rejection, Reply, reply::json, Reply};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
# 优化算法效率

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CartItem {
    id: u32,
    name: String,
    quantity: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Cart {
    items: Vec<CartItem>,
}

#[tokio::main]
async fn main() {
    let get_cart = warp::path("cart")
        .and(warp::get())
        .and_then(get_cart_handler);

    let add_to_cart = warp::path("cart")
# TODO: 优化性能
        .and(warp::post())
# TODO: 优化性能
        .and(with_cart())
        .and(warp::body::json())
        .and_then(add_to_cart_handler);

    let delete_from_cart = warp::path("cart")
        .and(warp::delete())
        .and(with_cart())
        .and(warp::path::param::<u32>())
        .and_then(delete_from_cart_handler);

    let routes = get_cart.or(add_to_cart).or(delete_from_cart);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Handler for GET /cart
# 改进用户体验
async fn get_cart_handler() -> Result<impl Reply, Rejection> {
    let cart = Cart {
        items: vec![
            CartItem { id: 1, name: "Apple".to_string(), quantity: 2 },
# FIXME: 处理边界情况
            CartItem { id: 2, name: "Banana".to_string(), quantity: 3 },
# 改进用户体验
        ],
    };

    Ok(reply::json(&cart))
# 添加错误处理
}

// Handler for POST /cart
# 增强安全性
async fn add_to_cart_handler(mut cart: Cart, new_item: CartItem) -> Result<impl Reply, Rejection> {
    for item in cart.items.iter_mut() {
        if item.id == new_item.id {
            item.quantity += new_item.quantity;
            return Ok(reply::json(&cart));
        }
    }

    cart.items.push(new_item);
    Ok(reply::json(&cart))
# 增强安全性
}

// Handler for DELETE /cart/:id
async fn delete_from_cart_handler(cart: Cart, item_id: u32) -> Result<impl Reply, Rejection> {
    let mut items = cart.items.into_iter().filter(|item| item.id != item_id).collect::<Vec<_>>();
    Ok(reply::json(&Cart { items }))
}

// Middleware to extract the cart from the request
fn with_cart() -> impl Filter<Extract = (Cart,), Error = Rejection> + Clone {
    warp::any().map(move || {
        let cart = Cart {
            items: vec![
                CartItem { id: 1, name: "Apple".to_string(), quantity: 2 },
                CartItem { id: 2, name: "Banana".to_string(), quantity: 3 },
# 优化算法效率
            ],
# 优化算法效率
        };
        cart
    }).boxed()
}
# TODO: 优化性能
