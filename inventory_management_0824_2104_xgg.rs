use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;

// 定义库存项
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    id: String,
    name: String,
    quantity: i32,
}

// 定义库存管理系统
struct InventoryManager {
    items: Arc<HashMap<String, Item>>,
}

impl InventoryManager {
    // 创建新的库存管理系统
    fn new() -> Self {
        Self {
            items: Arc::new(HashMap::new()),
        }
    }

    // 添加库存项
    fn add_item(&mut self, item: Item) {
        self.items.insert(item.id.clone(), item);
    }

    // 获取库存项
    fn get_item(&self, id: String) -> Option<Item> {
        self.items.get(&id).cloned()
    }

    // 更新库存项
    fn update_item(&mut self, item: Item) {
        self.items.insert(item.id.clone(), item);
    }

    // 删除库存项
    fn delete_item(&mut self, id: String) {
        self.items.remove(&id);
    }
}

// 定义API路由
fn routes(manager: Arc<InventoryManager>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("item")
        .and(warp::post())
        .and(with_manager(manager.clone()))
        .and(warp::body::json())
        .and_then(add_item)
        .or()
    warp::path!("item" / String)
        .and(warp::get())
        .and(with_manager(manager.clone()))
        .and_then(get_item)
        .or()
    warp::path!("item" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_manager(manager.clone()))
        .and_then(update_item)
        .or()
    warp::path!("item" / String)
        .and(warp::delete())
        .and(with_manager(manager.clone()))
        .and_then(delete_item)
}

// 添加库存项的处理器
async fn add_item(manager: Arc<InventoryManager>, item: Item) -> Result<impl warp::Reply, warp::Rejection> {
    manager.add_item(item);
    Ok(warp::reply::json(&item))
}

// 获取库存项的处理器
async fn get_item(manager: Arc<InventoryManager>, id: String) -> Result<impl warp::Reply, warp::Rejection> {
    match manager.get_item(id) {
        Some(item) => Ok(warp::reply::json(&item)),
        None => Err(warp::reject::not_found()),
    }
}

// 更新库存项的处理器
async fn update_item(manager: Arc<InventoryManager>, item: Item) -> Result<impl warp::Reply, warp::Rejection> {
    manager.update_item(item);
    Ok(warp::reply::json(&item))
}

// 删除库存项的处理器
async fn delete_item(manager: Arc<InventoryManager>, id: String) -> Result<impl warp::Reply, warp::Rejection> {
    manager.delete_item(id);
    Ok(warp::reply::json(&"Item deleted successfully"))
}

// 获取库存管理系统的引用
fn with_manager(manager: Arc<InventoryManager>) -> impl Filter<Extract = Arc<InventoryManager>, Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

#[tokio::main]
async fn main() {
    // 初始化库存管理系统
    let manager = Arc::new(InventoryManager::new());

    // 启动服务器
    let addr = "127.0.0.1:3030".parse().unwrap();
    let routes = routes(manager);
    warp::serve(routes).run(addr).await;
}
