// 引入所需的库
use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// 定义库存项目结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct InventoryItem {
    id: String,
    name: String,
    quantity: i32,
}

// 定义库存管理系统
struct InventoryManager {
    items: Arc<RwLock<Vec<InventoryItem>>>,
}

impl InventoryManager {
    // 创建新的库存管理系统
    pub fn new() -> InventoryManager {
        InventoryManager {
            items: Arc::new(RwLock::new(Vec::new())),
        }
    }

    // 添加库存项
    pub async fn add_item(&self, item: InventoryItem) -> Result<(), String> {
        let mut items = self.items.write().await;
        items.push(item);
        Ok(())
    }

    // 获取所有库存项
    pub async fn get_all_items(&self) -> Result<Vec<InventoryItem>, String> {
        let items = self.items.read().await;
        Ok(items.clone())
    }
}

// 定义API路由
fn create_routes(manager: Arc<InventoryManager>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let add_item_route = warp::post()
        .and(warp::path("add"))
        .and(warp::body::json())
        .and(with_manager(manager.clone()))
        .map(|item: InventoryItem| async move {
            manager.add_item(item).await.map_err(|e| warp::reject::custom(e))
        });

    let get_all_items_route = warp::get()
        .and(warp::path("items"))
        .and(with_manager(manager))
        .map(|manager: Arc<InventoryManager>| async move {
            manager.get_all_items().await.map_err(|e| warp::reject::custom(e))
        });

    add_item_route.or(get_all_items_route)
}

// 辅助函数，用于将库存管理系统注入请求处理函数
fn with_manager(manager: Arc<InventoryManager>) -> impl Filter<Extract = Arc<InventoryManager>, Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

#[tokio::main]
async fn main() {
    let manager = Arc::new(InventoryManager::new());
    let routes = create_routes(manager);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
