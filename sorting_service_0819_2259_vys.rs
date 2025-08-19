use warp::Filter;

// 定义排序算法结构体
struct SortingService;

impl SortingService {
    // 提供冒泡排序算法实现
    pub fn bubble_sort<T: Ord + Copy + std::iter::Sum<T>>(
        &self,
        list: Vec<T>,
    ) -> Result<Vec<T>, String> {
        if list.is_empty() {
            return Err("Input list is empty".to_string());
        }

        let mut sorted_list = list.clone();
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..sorted_list.len() {
                if sorted_list[i - 1] > sorted_list[i] {
                    sorted_list.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
        Ok(sorted_list)
    }

    // 提供快速排序算法实现
    pub fn quick_sort<T: Ord + Copy + std::iter::Sum<T>>(
        &self,
        list: Vec<T>,
    ) -> Result<Vec<T>, String> {
        if list.is_empty() {
            return Err("Input list is empty".to_string());
        }

        let mut sorted_list = list.clone();
        sorted_list.sort_unstable();
        Ok(sorted_list)
    }
}

// 定义路由和处理函数
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("sort" / "bubble")
        .and(warp::post())
        .and(warp::body::json::<Vec<u32>>())
        .and(with_sorting_service())
        .and_then(|service: SortingService, numbers: Vec<u32>| async move {
            match service.bubble_sort(numbers) {
                Ok(sorted) => warp::reply::json(&sorted),
                Err(e) => warp::reject::reject(),
            }
        })

    .or()

    warp::path!("sort" / "quick")
        .and(warp::post())
        .and(warp::body::json::<Vec<u32>>())
        .and(with_sorting_service())
        .and_then(|service: SortingService, numbers: Vec<u32>| async move {
            match service.quick_sort(numbers) {
                Ok(sorted) => warp::reply::json(&sorted),
                Err(e) => warp::reject::reject(),
            }
        })
}

// 将SortingService实例传递给路由
fn with_sorting_service() -> impl Filter<Extract = SortingService, Error = std::convert::Infallible> + Clone {
    warp::any().map(move || SortingService)
}

#[tokio::main]
async fn main() {
    println!("Server running on http://127.0.0.1:3030/");
    warp::serve(routes()).run(([127, 0, 0, 1], 3030)).await;
}