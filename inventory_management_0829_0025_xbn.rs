use warp::Filter;

// 定义库存项结构体
#[derive(Debug, Clone)]
struct InventoryItem {
    id: u32,
    name: String,
    quantity: u32,
}

// 创建库存管理系统结构体
struct InventoryManagement {
    items: Vec<InventoryItem>,
}

impl InventoryManagement {
    // 创建一个新的库存管理系统
    fn new() -> Self {
        InventoryManagement {
            items: vec![],
        }
    }

    // 添加库存项
    fn add_item(&mut self, item: InventoryItem) {
        self.items.push(item);
    }

    // 获取库存项
    fn get_item(&self, id: u32) -> Option<&InventoryItem> {
        self.items.iter().find(|i| i.id == id)
    }

    // 更新库存项数量
    fn update_item_quantity(&mut self, id: u32, quantity: u32) -> Result<(), String> {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.quantity = quantity;
            Ok(())
        } else {
            Err("Item not found".to_string())
        }
    }

    // 删除库存项
    fn remove_item(&mut self, id: u32) -> Result<(), String> {
        if let Some(index) = self.items.iter().position(|i| i.id == id) {
            self.items.remove(index);
            Ok(())
        } else {
            Err("Item not found".to_string())
        }
    }
}

// 定义API路由
fn inventory_api(inventory: warp:: Reply) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("inventory")
        .and(warp::post())
        .and(warp::body::json::<InventoryItem>())
        .and_then(move |item: InventoryItem| async move {
            inventory.add_item(item);
            Ok(warp::reply::json(&inventory.get_item(item.id).unwrap()))
        })
        .or(warp::path!("inventory" / u32)
            .and(warp::get())
            .and(warp::reply())
            .map(move |id: u32, inventory: warp::reply::Reply| async move {
                Ok(warp::reply::json(&inventory.get_item(id).unwrap()))
            }))
        .or(warp::path!("inventory" / u32 / "quantity")
            .and(warp::put())
            .and(warp::body::json::<u32>())
            .and(warp::reply())
            .and_then(move |id: u32, quantity: u32, inventory: warp::reply::Reply| async move {
                match inventory.update_item_quantity(id, quantity) {
                    Ok(_) => Ok(warp::reply::json(&inventory.get_item(id).unwrap())),
                    Err(e) => Err(warp::reject::custom(e)),
                }
            }))
        .or(warp::path!("inventory" / u32)
            .and(warp::delete())
            .and(warp::reply())
            .and_then(move |id: u32, inventory: warp::reply::Reply| async move {
                match inventory.remove_item(id) {
                    Ok(_) => Ok(warp::reply::""),
                    Err(e) => Err(warp::reject::custom(e)),
                }
            }))
}

// 主函数
#[tokio::main]
async fn main() {
    // 创建库存管理系统实例
    let inventory = InventoryManagement::new();

    // 设置WARP路由
    let routes = inventory_api(inventory);

    // 启动服务器
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
