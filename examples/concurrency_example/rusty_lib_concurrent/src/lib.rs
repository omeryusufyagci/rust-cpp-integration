use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

pub struct TemperatureSensor {
    buffer: Arc<(Mutex<Vec<f32>>, Condvar)>, // Thread-safe buffer
    new_data: Arc<(Mutex<bool>, Condvar)>, // Flag and condition variable to notify when new data is available
}

impl TemperatureSensor {
    pub fn new() -> Self {
        TemperatureSensor {
            buffer: Arc::new((Mutex::new(Vec::new()), Condvar::new())),
            new_data: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    pub fn start(&self, seed: u64) {
        let buffer = Arc::clone(&self.buffer);
        let new_data = Arc::clone(&self.new_data);
        let nb_iterations = 10;

        thread::spawn(move || {
            let mut rng = StdRng::seed_from_u64(seed); // Seeded RNG; use `rand::thread_rng();` for random data
            for _ in 0..nb_iterations {
                let temp = rng.gen_range(20.0..25.0);

                // Safely update the buffer and notify cpp-side
                {
                    let (lock, cvar) = &*buffer;
                    let mut buf = lock.lock().unwrap();
                    buf.push(temp);
                    println!("Rust: Generated temperature: {:.2}", temp);
                    cvar.notify_one(); // data-ready
                }

                // Safely update the flag and notify cpp-side
                {
                    let (lock, cvar) = &*new_data;
                    let mut flag = lock.lock().unwrap();
                    *flag = true;
                    cvar.notify_one(); // data-ready
                }

                // Introduce some sensor delay
                thread::sleep(Duration::from_millis(500));
            }
        });
    }

    pub fn get_readings(&self) -> Vec<f32> {
        // Safely access the buffer

        let (lock, _) = &*self.buffer;
        let buffer = lock.lock().unwrap();
        buffer.clone()
    }

    pub fn wait_for_data(&self) {
        // Wait for new sensor data from Rust-side

        let (lock, cvar) = &*self.new_data;
        let mut flag = lock.lock().unwrap();

        while !*flag {
            flag = cvar.wait(flag).unwrap();
        }
        *flag = false;
    }
}

#[no_mangle]
pub extern "C" fn generate_sensor() -> *mut TemperatureSensor {
    Box::into_raw(Box::new(TemperatureSensor::new()))
}

#[no_mangle]
pub extern "C" fn start_sensor(sensor: *mut TemperatureSensor, seed: u64) {
    let sensor = unsafe { &*sensor };
    sensor.start(seed);
}

#[no_mangle]
pub extern "C" fn get_readings(sensor: *mut TemperatureSensor) -> *mut f32 {
    let sensor = unsafe { &*sensor };
    let readings = sensor.get_readings();
    let mut vec = readings.into_boxed_slice();
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec); // no-dealloc by Rust; let C++ handle the heap allocation (C++ will ask Rust to free when it's done, see: `free_readings()`)
    ptr
}

#[no_mangle]
pub extern "C" fn get_readings_len(sensor: *mut TemperatureSensor) -> usize {
    let sensor = unsafe { &*sensor };
    sensor.get_readings().len()
}

#[no_mangle]
pub extern "C" fn wait_for_data(sensor: *mut TemperatureSensor) {
    let sensor = unsafe { &*sensor };
    sensor.wait_for_data();
}

#[no_mangle]
pub extern "C" fn free_readings(readings: *mut f32) {
    unsafe {
        drop(Box::from_raw(readings)); // Reclaim the memory allocated by C++ for the buffer
    }
}
