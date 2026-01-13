use tokio::select;
use tokio::time::{Duration, Instant, interval, sleep, timeout};
// Timeout concept

// async fn slow_operation() -> &'static str {
//     tokio::time::sleep(Duration::from_secs(3)).await;
//     "Operation Complete"
// }

// #[tokio::main]
// async fn main() {
//     let start = Instant::now();

//     match timeout(Duration::from_secs(3), slow_operation()).await {
//         Ok(result) => {
//             println!("Success: {} (Time: {:?})", result, start.elapsed());
//         }
//         Err(_elapsed) => {
//             println!("Timeout! (Time: {:?})", start.elapsed());
//         }
//     }
// }

// async fn flaky_api() -> String {
//     let delay = 2;
//     println!("API call started...(will take {} seconds", delay);
//     sleep(Duration::from_secs(delay)).await;
//     println!("API response received!");
//     "Data from API".to_string()
// }

// async fn fetch_with_retry() -> Result<String, String> {
//     let max_retries = 3;
//     let timeout_duration = Duration::from_secs(2);

//     for attempt in 1..=max_retries {
//         println!("\n Attempt {}/{}", attempt, max_retries);
//         match timeout(timeout_duration, flaky_api()).await {
//             Ok(data) => return Ok(data),
//             Err(_) => {
//                 println!("Timeout! Moving to next attempt...");
//             }
//         }
//     }

//     Err(format!("All {} attempts failed!", max_retries))
// }

// #[tokio::main]
// async fn main() {
//     let start = Instant::now();

//     match fetch_with_retry().await {
//         Ok(data) => {
//             println!("\n Success: {} (Total time: {:?})", data, start.elapsed())
//         }
//         Err(e) => {
//             println!("\n Failed: {} (Total time: {:?}", e, start.elapsed());
//         }
//     }
// }

// =================================================//
// Interval Concept

// async fn do_work() {
//     println!("Doing some work at {:?}", Instant::now());
// }
// #[tokio::main]
// async fn main() {
//     loop {
//         do_work().await;
//         sleep(Duration::from_secs(5)).await;
//     }

//     let mut ticker = interval(Duration::from_secs(5));
//     loop {
//         ticker.tick().await;
//         do_work().await
//     }

//     let start = Instant::now();
//     let mut ticker = interval(Duration::from_millis(500));

//     for i in 1..=5 {
//         ticker.tick().await;
//         println!("Tick {}: {:?} elapsed", i, start.elapsed());
//     }
// }

// ===================== //

// async fn check_server_health() {
//     println!(" Checking server...");
//     tokio::time::sleep(Duration::from_millis(100)).await;
//     println!(" Server is healthy");
// }

// #[tokio::main]
// async fn main() {
//     let mut ticker = interval(Duration::from_secs(2));

//     println!("Starting health checks every 2 seconds...\n");

//     for i in 1..=5 {
//         ticker.tick().await;
//         let start = Instant::now();
//         check_server_health().await;
//         println!("  (took {:?})\n", start.elapsed());
//     }
//     println!("Health checks complete");
// }

// ===================== //

// async fn check_api() -> Duration {
//     let delay = 950;
//     sleep(Duration::from_millis(delay)).await;
//     Duration::from_millis(delay)
// }
// #[tokio::main]
// async fn main() {
//     let mut ticker = interval(Duration::from_secs(2));

//     println!("Starting API monitoring (every 3 seconds)...\n");

//     for check_num in 1..=10 {
//         ticker.tick().await;
//         let res_time = check_api().await;

//         if res_time > Duration::from_secs(1) {
//             println!("Check #{} WARNING: slow repsonse ({:?})", check_num, res_time);
//         } else {
//             println!("Check #{} OK ({:?})", check_num, res_time);
//         }
//     }

//     println!("\nMonitoring complete");
// }

// ===================== //

// JOIN and SPAWN concept mainly Join

// async fn fetch_user(id: u32) -> String {
//     println!("Fetching user {}...", id);
//     sleep(Duration::from_secs(1)).await;
//     println!("  User {} done!", id);
//     format!("User {}", id)
// }

// async fn fetch_posts(id: u32) -> String {
//     println!("Fetching posts for user {}...", id);
//     sleep(Duration::from_secs(3)).await;
//     println!("  Posts {} done!",id);
//     format!("Posts for user {}", id)
// }

// async fn fetch_comments(id: u32) -> String {
//     println!("  Fetching comments {}...", id);
//     sleep(Duration::from_secs(2)).await;
//     println!(" Comments {} done!", id);
//     format!("{} comments", id)
// }

// #[tokio::main]
// async fn main() {
//     let start = Instant::now();

//     println!("Starting all 3 tasks...\n");

//     let (user, posts, comments) = join!(fetch_user(1), fetch_posts(1), fetch_comments(1));
//     println!("\n{}", user);
//     println!("{}", posts);
//     println!("{}", comments);
//     println!("Total time: {:?}", start.elapsed());

// }

// ===================== //

// Select concept

async fn fast_api() -> &'static str {
    println!("[Fast API] Starting...");
    sleep(Duration::from_secs(4)).await;
    println!("[Fast API] Done!");
    "Fast API response"
}

async fn slow_api() -> &'static str {
    println!("[Slow API] starting...");
    sleep(Duration::from_secs(3)).await;
    println!("[Slow API] Done");
    "Slow API response"
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    println!("=== RACE STARTED ===\n");

    select! {
        result = fast_api() => {
        println!("\n🏆 Winner: {}", result);
    }
    result = slow_api() => {
        println!("\n🏆 Winner: {}", result);
    }
    }
    println!("Total time: {:?}", start.elapsed());
    println!("\n(Loser was cancelled!)");
}
