use warp::Filter;
use serde::Deserialize;
use std::sync::Arc;
use std::collections::HashMap;

// Define a struct to represent an item in the inventory.
#[derive(Clone, Deserialize, Debug)]
struct InventoryItem {
    id: u32,
    name: String,
    quantity: u32,
}

// In-memory storage for our inventory items.
// This can be replaced with a database in a real-world application.
#[derive(Clone, Default)]
struct Inventory {
    items: HashMap<u32, InventoryItem>,
# 增强安全性
}
# 添加错误处理

// Implementing methods for our Inventory struct.
impl Inventory {
# TODO: 优化性能
    // Adds a new item to the inventory.
    fn add_item(&mut self, item: InventoryItem) {
        self.items.insert(item.id, item);
    }

    // Retrieves an item from the inventory by ID.
    fn get_item(&self, id: u32) -> Option<&InventoryItem> {
        self.items.get(&id)
    }
# FIXME: 处理边界情况

    // Updates an existing item in the inventory.
    fn update_item(&mut self, item: InventoryItem) {
        self.items.insert(item.id, item);
    }

    // Removes an item from the inventory.
    fn remove_item(&mut self, id: u32) -> Option<InventoryItem> {
        self.items.remove(&id)
    }
}

#[tokio::main]
# NOTE: 重要实现细节
async fn main() {
    // Initialize the in-memory inventory.
    let inventory = Arc::new(Inventory::default());

    // Define a filter for adding a new item to the inventory.
    let add_item = warp::post()
        .and(warp::path("add"))
        .and(with_inventory(inventory.clone()))
        .and(warp::body::json())
        .map(|inventory: Arc<Inventory>, item: InventoryItem| {
            inventory.add_item(item);
            warp::reply::json(&item)
        });
# TODO: 优化性能

    // Define a filter for getting an item from the inventory by ID.
    let get_item = warp::get()
        .and(warp::path("get"))
        .and(warp::path::param::<u32>())
        .and(with_inventory(inventory.clone()))
        .map(|id: u32, inventory: Arc<Inventory>| {
            match inventory.get_item(id) {
                Some(item) => warp::reply::json(item),
                None => warp::reply::with_status(warp::reply::json(&"Item not found"), warp::http::StatusCode::NOT_FOUND),
            }
        });

    // Define a filter for updating an item in the inventory.
    let update_item = warp::post()
        .and(warp::path("update"))
        .and(warp::path::param::<u32>())
        .and(with_inventory(inventory.clone()))
        .and(warp::body::json())
        .map(|id: u32, inventory: Arc<Inventory>, item: InventoryItem| {
            let mut updated_item = item.clone();
            updated_item.id = id;
            if inventory.update_item(updated_item) {
# TODO: 优化性能
                warp::reply::json(&item)
            } else {
                warp::reply::with_status(warp::reply::json(&"Item not found"), warp::http::StatusCode::NOT_FOUND)
            }
# 优化算法效率
        });

    // Define a filter for removing an item from the inventory.
    let remove_item = warp::delete()
        .and(warp::path("remove"))
        .and(warp::path::param::<u32>())
        .and(with_inventory(inventory.clone()))
        .map(|id: u32, inventory: Arc<Inventory>| {
            match inventory.remove_item(id) {
                Some(item) => warp::reply::json(&item),
# 添加错误处理
                None => warp::reply::with_status(warp::reply::json(&"Item not found"), warp::http::StatusCode::NOT_FOUND),
            }
        });

    // Combine all the filters and start the server.
    let routes = add_item.or(get_item).or(update_item).or(remove_item);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
# 添加错误处理
}

// A helper function to extract the inventory from the request.
fn with_inventory(
    inventory: Arc<Inventory>,
) -> impl Filter<Extract = (Arc<Inventory>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || inventory.clone())
}