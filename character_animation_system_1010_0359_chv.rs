use warp::Filter;

// 定义角色动画系统的状态结构
struct AnimationState {
    animations: Vec<Animation>,
}

// 定义动画结构
struct Animation {
    name: String,
    frames: Vec<String>,
    current_frame: usize,
}

impl AnimationState {
    // 创建一个新的动画状态
    pub fn new() -> Self {
        AnimationState {
            animations: Vec::new(),
        }
    }

    // 添加一个新的动画
    pub fn add_animation(&mut self, animation: Animation) {
        self.animations.push(animation);
    }

    // 更新动画帧
    pub fn update(&mut self) {
        for animation in &mut self.animations {
            animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
        }
    }

    // 获取当前帧的图像
    pub fn get_current_frame(&self, animation_name: &str) -> Option<String> {
        for animation in &self.animations {
            if animation.name == animation_name {
                return Some(animation.frames[animation.current_frame].clone());
            }
        }
        None
    }
}

// 创建路由处理函数
fn route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("animation" / String / "frame")
        .map(|animation_name: String| {
            let mut animation_state = AnimationState::new();
            animation_state.add_animation(Animation {
                name: animation_name.clone(),
                frames: vec!["frame1.png".to_string(), "frame2.png".to_string()],
                current_frame: 0,
            });

            animation_state.update();
            match animation_state.get_current_frame(&animation_name) {
                Some(frame) => warp::reply::json(&frame),
                None => warp::reply::json(&"Animation not found".to_string()),
            }
        })
}

// 启动服务器
fn main() {
    let route = route();
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}