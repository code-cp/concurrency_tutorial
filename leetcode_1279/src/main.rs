use std::time::Duration; 
use std::sync::{Arc, Mutex}; 

use leetcode_1279::TrafficLight; 

#[tokio::main]
async fn main() {
    let traffic_light = Arc::new(Mutex::new(TrafficLight::new())); 
    let cars = vec![1, 3, 5, 2, 4];
    let directions = vec![2, 1, 2, 4, 3];
    let arrival_times = vec![10, 20, 30, 40, 50];

    for i in 0..cars.len() {
        let car_id = cars[i]; 
        let direction = directions[i]; 
        let road_id = if direction == 1 || direction == 2 {1} else {2}; 
        let arrival_time = arrival_times[i]; 

        let traffic_light_clone = Arc::clone(&traffic_light);

        tokio::task::spawn_blocking(move || {
            std::thread::sleep(Duration::from_millis(arrival_time as u64));

            let mut traffic_light_guard = traffic_light_clone.lock().unwrap();
            traffic_light_guard.car_arrived(
                road_id, 
                || {println!("Traffic Light On Road {} Is Green", road_id);}, 
                || {println!("Car {} Has Passed Road {} In Direction {}", car_id, road_id, direction);}, 
            ); 
        });
    }

    tokio::time::sleep(Duration::from_millis(arrival_times.last().unwrap() + 100)).await; 
}