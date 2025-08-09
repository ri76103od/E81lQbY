// audit_log_service.rs
use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use log::info;

// 定义安全审计日志条目的结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct AuditLog {
    id: u64,
    action: String,
    user_id: u64,
    timestamp: String,
    ip: String,
}

// 模拟的安全审计日志存储
struct AuditLogService {
    logs: Arc<Mutex<HashMap<u64, AuditLog>>>,
    next_id: u64,
}

impl AuditLogService {
    // 初始化一个新的安全审计日志服务
    fn new() -> Self {
        AuditLogService {
            logs: Arc::new(Mutex::new(HashMap::new())),
            next_id: 0,
        }
    }

    // 添加一个新的日志条目
    fn add_log(&mut self, action: String, user_id: u64, ip: String) -> AuditLog {
        let log = AuditLog {
            id: self.next_id,
            action,
            user_id,
            timestamp: format!("{}", Utc::now()),
            ip,
        };
        self.next_id += 1;
        let mut logs = self.logs.lock().unwrap();
        logs.insert(self.next_id - 1, log.clone());
        log
    }
}

// 定义Warp路由
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let audit = warp::any().map(move || {
        let service = Arc::new(AuditLogService::new());
        let service_clone = service.clone();
        let route_add_log = warp::path("log")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_service(service_clone))
            .map(|action: String, user_id: u64, ip: String| {
                let log = service.add_log(action, user_id, ip);
                info!("Added log: {:?}