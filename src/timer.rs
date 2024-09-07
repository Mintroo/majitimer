use std::time;

#[derive(Debug)]
pub struct Timer {
    start: Option<time::Instant>,
    elapsed_time: time::Duration,
    paused: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: None,
            elapsed_time: time::Duration::default(),
            paused: true,
        }
    }
    pub fn init(&mut self) {
        self.elapsed_time = time::Duration::default();
        self.start = Some(time::Instant::now());
        self.paused = false;
    }
    /// # Safety
    /// `init`メソッドを呼び出した後
    pub fn pause(&mut self) {
        self.elapsed_time += self.start.unwrap().elapsed();
        self.start = None;
        self.paused = true;
    }
    pub fn resume(&mut self) {
        self.start = Some(time::Instant::now());
        self.paused = false;
    }
    pub fn get_time(&self) -> time::Duration {
        if self.paused {
            self.elapsed_time
        } else {
            self.elapsed_time + self.start.unwrap().elapsed()
        }
    }
    pub fn is_paused(&self) -> bool {
        self.paused
    }
}

pub trait MyToType {
    fn to_time_string(&self) -> String;
}

impl MyToType for time::Duration {
    fn to_time_string(&self) -> String {
        let all_times = self.as_secs();

        let days = all_times / 86400;
        let hours = all_times / 3600;
        let mins = (all_times % 3600) / 60;
        let secs = all_times % 60;

        format!("{:01}:{:02}:{:02}:{:02}", days, hours, mins, secs)
    }
}

#[derive(Debug)]
pub struct CountDownTimer {
    core: Timer,
    limit: time::Duration,
}

impl CountDownTimer {
    pub fn new() -> Self {
        Self {
            core: Timer::new(),
            limit: time::Duration::default(),
        }
    }
    pub fn init(&mut self, limit: time::Duration) {
        self.limit = limit;
        self.core.init();
    }
    pub fn pause(&mut self) {
        self.core.pause();
    }
    pub fn resume(&mut self) {
        self.core.resume();
    }
    pub fn get_time(&self) -> time::Duration {
        if self.limit >= self.core.get_time() {
            self.limit - self.core.get_time()
        } else {
            time::Duration::ZERO
        }
    }
    pub fn is_paused(&self) -> bool {
        self.core.is_paused()
    }
}
