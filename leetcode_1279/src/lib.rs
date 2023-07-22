use std::sync::{Arc, Mutex}; 
use std::time::Duration; 

pub struct TrafficLight {
    lock: Arc<Mutex<()>>, 
    road_id: i32, 
}

impl TrafficLight {
    pub fn new() -> Self {
        let road_id = 1; 
        println!("Traffic Light On Road {} Is Green", road_id);
        TrafficLight {
            lock: Arc::new(Mutex::new(())), 
            road_id, 
        }
    }

    pub fn car_arrived(
        &mut self, 
        road_id: i32, 
        turn_green: impl Fn() -> (), 
        cross_car: impl Fn() -> (), 
    ) {
        let _lock = self.lock.lock().unwrap(); 
        if self.road_id != road_id {
            self.road_id = road_id; 
            turn_green(); 
            drop(_lock); 
        }
        cross_car(); 
    }
}
