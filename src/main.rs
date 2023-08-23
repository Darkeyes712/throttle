mod throttle;
// use ratelimit_meter::{DirectRateLimiter, NonZeroU32, RateLimiter};
use std::time::Duration;

fn generate_simulated_data() -> impl Iterator<Item = usize> {
    (1..=10).cycle() // Creates an infinite loop of data: 1, 2, ..., 10, 1, 2, ...
}

fn main() {
    let per_second_limit = 1; // Allow 2 operations per second
    let throttle_interval = Duration::from_millis(1000); // Throttle to 500 milliseconds intervals

    let mut throttle = throttle::Throttle::new(throttle_interval);
    let simulated_data = generate_simulated_data();

    for data in simulated_data.take(20) {
        if !throttle.execute(per_second_limit) {
            println!("Rate limit exceeded! Breaking out of loop.");
            break;
        }
        println!("Processing data: {:?}", data);
    }
}
