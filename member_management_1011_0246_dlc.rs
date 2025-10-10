use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Rejection;

// Define the Member struct which will be used for serialization and deserialization.
#[derive(Serialize, Deserialize, Clone, Debug)]
# 增强安全性
struct Member {
# TODO: 优化性能
    id: i32,
# FIXME: 处理边界情况
    name: String,
    email: String,
}

// This will hold our in-memory 'database'.
#[derive(Default)]
# 扩展功能模块
struct MemberDatabase {
    members: Vec<Member>,
}

// Implementing methods to interact with the 'database'.
impl MemberDatabase {
    fn find_member(&self, id: i32) -> Option<&Member> {
        self.members.iter().find(|m| m.id == id)
    }
# NOTE: 重要实现细节

    fn add_member(&mut self, member: Member) -> Result<(), String> {
        if self.members.iter().any(|m| m.id == member.id) {
            Err("Member already exists".to_string())
        } else {
            self.members.push(member);
            Ok(())
        }
    }
}
# 增强安全性

// The application state containing our in-memory database.
#[derive(Clone)]
struct AppState {
    database: Arc<RwLock<MemberDatabase>>,
}
# 添加错误处理

// Filters to handle different routes and logic.
fn member_api() -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    let state = AppState {
        database: Arc::new(RwLock::new(MemberDatabase::default())),
    };
# NOTE: 重要实现细节

    let get_member = warp::get()
        .and(warp::path!("member" / i32))
        .and(with_state.clone())
        .and_then(|id: i32, state: AppState| async move {
# NOTE: 重要实现细节
            let db = state.database.read().await;
            match db.find_member(id) {
                Some(member) => Ok(warp::reply::json(&member)),
# 优化算法效率
                None => Err(warp::reject::not_found()),
            }
        });

    let add_member = warp::post()
        .and(warp::path("member"))
        .and(warp::body::json::<Member>())
        .and(with_state.clone())
        .and_then(|member: Member, state: AppState| async move {
# FIXME: 处理边界情况
            let mut db = state.database.write().await;
            match db.add_member(member) {
                Ok(_) => Ok(warp::reply::json(&member)),
                Err(e) => Err(warp::reject::custom(MemberError(e))),
            }
# NOTE: 重要实现细节
        });

    get_member.or(add_member)
}
# FIXME: 处理边界情况

#[tokio::main]
async fn main() {
# 优化算法效率
    let with_state = warp::any().map(move || AppState {
        database: Arc::new(RwLock::new(MemberDatabase::default())),
    });

    let api = member_api();
    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Custom rejection type for member-related errors.
#[derive(Debug)]
struct MemberError(String);

impl warp::reject::Reject for MemberError {}
