pub struct App {
    pub running: bool,
    pub cpu_usage: u8,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            cpu_usage: 0,
        }
    }

    pub fn tick(&mut self) {
        self.cpu_usage = (self.cpu_usage + 1) % 100;
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}