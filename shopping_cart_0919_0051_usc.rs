use warp::Filter;

// 定义购物车项结构体
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct CartItem {
    product_id: String,
    quantity: u32,
}

// 定义购物车结构体
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
struct ShoppingCart {
    items: Vec<CartItem>,
}

// 实现添加商品到购物车的功能
async fn add_to_cart(cart: ShoppingCart, item: CartItem) -> Result<ShoppingCart, warp::Rejection> {
    // 查找购物车中是否已有该商品
    let mut updated_cart = cart;
    let mut item_found = false;
    for existing_item in &mut updated_cart.items {
        if existing_item.product_id == item.product_id {
            // 商品存在，增加数量
            existing_item.quantity += item.quantity;
            item_found = true;
            break;
        }
    }

    if !item_found {
        // 商品不存在，添加到购物车
        updated_cart.items.push(item);
    }

    Ok(updated_cart)
}

// 实现购物车的HTTP接口
fn shopping_cart_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let add_to_cart_route = warp::post()
        .and(warp::path("cart"))
        .and(warp::body::json())
        .and(with_shopping_cart())
        .and_then(|item: CartItem, cart: ShoppingCart| async move {
            match add_to_cart(cart, item).await {
                Ok(updated_cart) => warp::reply::json(&updated_cart),
                Err(_) => warp::reject::custom(ShoppingCartError::InternalServerError),
            }
        });

    add_to_cart_route
}

// 购物车服务的错误类型
#[derive(Debug)]
enum ShoppingCartError {
    InternalServerError,
}

impl warp::reject::Reject for ShoppingCartError {}

// 用于测试的辅助函数，将购物车初始化为默认值
fn with_shopping_cart() -> impl Filter<Extract = ShoppingCart, Error = warp::Rejection> + Clone {
    warp::any().map(|| ShoppingCart::default())
}

// 启动服务器
#[tokio::main]
async fn main() {
    let routes = shopping_cart_routes();
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 实现错误转换为HTTP响应的功能
impl warp::reply::Reply for ShoppingCartError {
    fn into_response(self) -> warp::reply::Response {
        match self {
            ShoppingCartError::InternalServerError => warp::reply::with_status(
                "Internal Server Error",
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

// 测试购物车功能
#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::WarpTest;
    use serde_json::json;

    #[tokio::test]
    async fn test_add_to_cart() {
        let test_server = WarpTest::new();
        let cart_item = CartItem {
            product_id: "123".to_string(),
            quantity: 1,
        };
        let initial_cart = ShoppingCart {
            items: vec![],
        };

        let response = test_server
            .post()
            .json(&cart_item)
            .reply(&warp::test::request().path("/cart"), &initial_cart);

        let body = response
            .await
            .json::<Vec<CartItem>>()
            .await
            .unwrap();

        assert_eq!(body.len(), 1);
    }
}
