// use tokio::select;
// use tokio::time::{Duration, Instant, interval, sleep, timeout};
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

// async fn fast_api() -> &'static str {
//     println!("[Fast API] Starting...");
//     sleep(Duration::from_secs(4)).await;
//     println!("[Fast API] Done!");
//     "Fast API response"
// }

// async fn slow_api() -> &'static str {
//     println!("[Slow API] starting...");
//     sleep(Duration::from_secs(3)).await;
//     println!("[Slow API] Done");
//     "Slow API response"
// }

// #[tokio::main]
// async fn main() {
//     let start = Instant::now();

//     println!("=== RACE STARTED ===\n");

//     select! {
//         result = fast_api() => {
//         println!("\n🏆 Winner: {}", result);
//     }
//     result = slow_api() => {
//         println!("\n🏆 Winner: {}", result);
//     }
//     }
//     println!("Total time: {:?}", start.elapsed());
//     println!("\n(Loser was cancelled!)");
// }

// ===================== //
// ===================== //
// ===================== //

//  DRILL 1: Concurrent API Calls
// Problem Statement:
// Call 3 different APIs concurrently (not sequentially). Measure and compare:

// Sequential execution time (one after another)
// Concurrent execution time (all together)
// Print which API finished first, second, third

// use tokio::join;
// use tokio::time::{Duration, Instant, sleep};

// // first API which will take 2 seconds
// async fn fetch_users() -> String {
//     println!("API 1 Starting users fecth...");
//     sleep(Duration::from_secs(2)).await;
//     println!("API 1 Users fetched");
//     "Users data".to_string()
// }

// // second API which will take 3 seconds
// async fn fetch_posts() -> String {
//     println!("API 2 Starting posts fecth...");
//     sleep(Duration::from_secs(3)).await;
//     println!("API 2 posts fecthed!");
//     "Posts data".to_string()
// }

// // third API takes 1 second
// async fn fetch_settings() -> String {
//     println!("API 3 Starting settings fecth...");
//     sleep(Duration::from_secs(1)).await;
//     println!("API 3 settings fecthed!");
//     "Settings data".to_string()
// }

// #[tokio::main]
// async fn main() {
//     // part1 = sequential execution which is normal way to do things
//     println!("=== Part 1: Sequential one by one ===");

//     let start = Instant::now();

//     let users = fetch_users().await;
//     let posts = fetch_posts().await;
//     let settings = fetch_settings().await;

//     let sequential_time = start.elapsed();
//     println!("\nSequential Results:");
//     println!("  users: {}", users);
//     println!("  posts: {}", posts);
//     println!("  settings: {}", settings);
//     println!("  Total time: {:?}", sequential_time);

//     // part 2 concurrent executioon an efficient way to do this
//     println!("=== Part 2 Cncurrent (parallel) execution ===");
//     let start = Instant::now();

//     let (users, posts, settings) = join!(fetch_users(), fetch_posts(), fetch_settings());

//     let concurrent_time = start.elapsed();

//     println!("\nConcurrent Results:");
//     println!("\n{}", users);
//     println!("{}", posts);
//     println!("{}", settings);
//     println!("Total time: {:?}", concurrent_time);

//     println!("\n--- Comparison ---");
//     let time_saved = sequential_time - concurrent_time;
//     println!("Time Saved {:?}", time_saved);
//     println!("speed improvement: {}x faster", sequential_time.as_secs_f64() / concurrent_time.as_secs_f64())

// }

// ===================== //
// ===================== //

// DRILL 2: Race with Timeout
// Problem Statement:
// Call an API with a 3-second timeout. The API might take 2 seconds or 5 seconds randomly.

// If API responds within timeout → Success ✅
// If API takes too long → Timeout error ❌
// Test both scenarios (fast response & slow response)
// Track which scenario occurred and how long it took

// use tokio::select;
// use tokio::time::{Duration, Instant, sleep};

// // this api will simulate an API that might be slow or fast

// async fn unreliable_api(delay_sec: u64) -> String {
//     println!(" API Starting request");
//     sleep(Duration::from_secs(delay_sec)).await;
//     println!(" API Response ready!");
//     format!("API data (took {}s)", delay_sec)
// }

// async fn timeout_after(secs: u64) {
//     sleep(Duration::from_secs(secs)).await;
// }

// #[tokio::main]
// async fn main() {
//     println!("=== Race with Timeout ===");

//     println!("=== Test 1: Fast API (2s with 3s timeout) ---");
//     let start = Instant::now();

//     select! {
//         result = unreliable_api(2) => {
//             let elapsed = start.elapsed();
//             println!("API succeeded in {:.2}s: {}", elapsed.as_secs_f32(), result);
//         }
//         result = timeout_after(3) => {
//             let elapsed = start.elapsed();
//         println!("Timed out after {:.2}s: {:?} (API took too long)", elapsed.as_secs_f32(), result);
//     }
//     }
//     println!("Elapsed: {:?}\n", start.elapsed());

//     // Test 2 slow API should Timeout
//     let start = Instant::now();
//     select! {
//         result = unreliable_api(5) => {
//             let elapsed = start.elapsed();
//             println!("API succeeded in {:.2}s: {}", elapsed.as_secs_f32(), result);
//         }
//         result = timeout_after(3) => {
//             let elapsed = start.elapsed();
//         println!("Timed out after {:.2}s: {:?} (API took too long)", elapsed.as_secs_f32(), result);
//     }
//     }
//     println!("Elapsed: {:?}\n", start.elapsed());

//     // Test 3 Edge case (exactly at timeout)
//     let start = Instant::now();
//     select! {
//         result = unreliable_api(3) => {
//             let elapsed = start.elapsed();
//             println!("API succeeded in {:.2}s: {}", elapsed.as_secs_f32(), result);
//         }
//         result = timeout_after(3) => {
//             let elapsed = start.elapsed();
//         println!("Timed out after {:.2}s: {:?} (API took too long)", elapsed.as_secs_f32(), result);
//     }
//     }
//     println!("Elapsed: {:?}\n", start.elapsed());
// }

// ===================== //
// ===================== //

// 🚀 DRILL 3: Retry Logic with Exponential Backoff
// Problem Statement:
// Call a flaky API that fails randomly. Implement retry logic with exponential backoff:

// Retry up to 5 times
// Wait time increases exponentially: 1s → 2s → 4s → 8s → 16s
// If API succeeds, return immediately
// If all retries fail, return error
// Track total time and which attempt succeeded

// use tokio::time::{Duration, Instant, sleep};

// // this simulates flaky API (50% success rate)

// async fn flaky_api(attempt: u32) -> Result<String, String> {
//     println!("  [Attempt {}] Calling API...", attempt);
//     sleep(Duration::from_millis(500)).await;

//     if attempt < 4 {
//         println!("  [Attempt {}] Failed", attempt);
//         return Err("API temporarily unavailable".to_string());
//     }
//     println!("  [Attempt {}] Success", attempt);
//     Ok(format!("Data from attempt {}", attempt))
// }

// async fn retry_with_backoff() -> Result<String, String> {
//     let max_retries = 5;
//     let base_delay_secs = 1;

//     for attempt in 1..=max_retries {
//         println!("\n Retry attempt {}/{}", attempt, max_retries);
//         match flaky_api(attempt).await {
//             Ok(data) => return Ok(data),
//             Err(e) => {
//                 println!("  Error: {}", e);
//                 if attempt < max_retries {
//                     let delay = base_delay_secs * 2u64.pow(attempt - 1);
//                     println!("  Waiting {}s before retry...", delay);
//                     sleep(Duration::from_secs(delay)).await;
//                 }
//             }
//         }
//     }
//     Err("All retries exhausted!".to_string())
// }

// #[tokio::main]

// async fn main() {
//     let start = Instant::now();

//     match retry_with_backoff().await {
//         Ok(data) => {
//             println!("\nFinal Result: {}", data);
//             println!("Total time: {:?}", start.elapsed());
//         }
//         Err(e) => {
//             println!("\nFinal Result: {}", e);
//             println!("Total time: {:?}", start.elapsed());

//         }
//     }
// }

// ===================== //
// ===================== //



// example 1
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let url = "https://httpbin.org/get";
//     let response = reqwest::get(url).await?;

//     println!("Status: {}", response.status());

//     let body = response.text().await?;
//     println!("Body: {}", body);
//     Ok(())
// }

// example 2

use serde::{Deserialize};
#[derive(Deserialize, Debug)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    let post: Post = reqwest::get(url)
    .await?
    .json()
    .await?;

    println!("Post: {:#?}", post);
    Ok(())
}

// example 3
// use reqwest;
// use serde::{Deserialize, Serialize};

// #[derive(Serialize)]
// struct CreatePost {
//     title: String,
//     body: String,
//     user_id: u32,
// }

// #[derive(Deserialize, Debug)]
// struct CreatedPost {
//     id: u32,
//     title: String,
//     body: String,
//     user_id: u32,
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = reqwest::Client::new();

//     let new_post = CreatePost {
//         title: "My Post".to_string(),
//         body: "This is the body".to_string(),
//         user_id: 1,
//     };

//     let response: CreatedPost = client
//     .post("https://jsonplaceholder.typicode.com/posts")
//     .json(&new_post)
//     .send()
//     .await?
//     .json()
//     .await?;

//     println!("Created: {:#?}", response);

//     Ok(())
// }