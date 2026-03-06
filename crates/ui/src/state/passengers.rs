pub struct State {
    pub adults: u8,
    pub children: u8,
}

impl Default for State {
    fn default() -> Self {
        Self {
            adults: 1,
            children: 0,
        }
    }
}

impl State {
    pub fn total(&self) -> u8 {
        self.adults + self.children
    }

    pub fn add_adult(&mut self) {
        if self.total() < 9 {
            self.adults += 1;
        }
    }

    pub fn add_child(&mut self) {
        if self.total() < 9 {
            self.children += 1;
        }
    }

    pub fn remove_adult(&mut self) {
        if self.adults > 0 && self.total() > 1 {
            self.adults -= 1;
        }
    }

    pub fn remove_child(&mut self) {
        if self.children > 0 && self.total() > 1 {
            self.children -= 1;
        }
    }
}
