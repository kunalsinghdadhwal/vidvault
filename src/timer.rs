use std::time::Instant;

pub struct Timer {
    title: &'static str,
    time: Instant,
}

impl Drop for Timer {
    fn drop(&mut self) {
        let micros = self.time.elapsed().as_micros();
        let millis = self.time.elapsed().as_millis();

        if micros < 10000 {
            println!("{} completed in {}μs", self.title, micros);
        } else {
            println!("{} completed in {}ms", self.title, millis);
        }
    }
}

impl Timer {
    pub fn new(title: &'static str) -> Self {
        println!("{} started", title);
        Timer {
            title,
            time: Instant::now(),
        }
    }
}
