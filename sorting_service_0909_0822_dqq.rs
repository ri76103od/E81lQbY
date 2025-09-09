use warp::Filter;

/// 定义一个排序算法的枚举，包含不同的排序实现
enum SortingAlgorithm {
    BubbleSort,
    SelectionSort,
    InsertionSort,
}

/// 排序函数
/// 根据排序算法枚举对向量进行排序
fn sort_numbers(numbers: Vec<i32>, algorithm: SortingAlgorithm) -> Vec<i32> {
    match algorithm {
        SortingAlgorithm::BubbleSort => bubble_sort(&numbers),
        SortingAlgorithm::SelectionSort => selection_sort(&numbers),
        SortingAlgorithm::InsertionSort => insertion_sort(&numbers),
    }
}

/// 冒泡排序
fn bubble_sort(numbers: &Vec<i32>) -> Vec<i32> {
    let mut numbers = numbers.clone();
    let len = numbers.len();
    for _ in 0..len {
        for j in 1..len {
            if numbers[j - 1] > numbers[j] {
                numbers.swap(j - 1, j);
            }
        }
    }
    numbers
}

/// 选择排序
fn selection_sort(numbers: &Vec<i32>) -> Vec<i32> {
    let mut numbers = numbers.clone();
    let len = numbers.len();
    for i in 0..len {
        let mut min_index = i;
        for j in i + 1..len {
            if numbers[j] < numbers[min_index] {
                min_index = j;
            }
        }
        numbers.swap(min_index, i);
    }
    numbers
}

/// 插入排序
fn insertion_sort(numbers: &Vec<i32>) -> Vec<i32> {
    let mut numbers = numbers.clone();
    let len = numbers.len();
    for i in 1..len {
        let current = numbers[i];
        let mut position = i;
        while position > 0 && numbers[position - 1] > current {
            numbers[position] = numbers[position - 1];
            position -= 1;
        }
        numbers[position] = current;
    }
    numbers
}

/// 创建一个Warp过滤器来处理排序请求
fn sorting_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("sort")
        .and(warp::post())
        .and(warp::json())
        .and(warp::header::<String>::header("sorting_algorithm"))
        .and_then(|numbers: Vec<i32>, algorithm: String| {
            let algorithm = match algorithm.as_str() {
                "bubble" => SortingAlgorithm::BubbleSort,
                "selection" => SortingAlgorithm::SelectionSort,
                "insertion" => SortingAlgorithm::InsertionSort,
                _ => return warp::reject::not_found(),
            };
            warp::reply::json(&sort_numbers(numbers, algorithm))
        })
}

/// Warp初始化函数
fn main() {
    let sorting_route = sorting_route();
    warp::serve(sorting_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
