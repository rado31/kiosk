pub struct State {
    pub adults: i8,
    pub children: i8,
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
    pub fn total(&self) -> i8 {
        self.adults + self.children
    }

    fn can_add(&self) -> bool {
        self.total() < 9
    }

    fn can_remove(&self, new_total: i8) -> bool {
        new_total >= 0 && self.total() > 1
    }

    pub fn add_adult(&mut self) {
        if self.can_add() {
            self.adults += 1;
        }
    }

    pub fn add_child(&mut self) {
        if self.can_add() {
            self.children += 1;
        }
    }

    pub fn remove_adult(&mut self) {
        if self.can_remove(self.adults - 1) {
            self.adults -= 1;
        }
    }

    pub fn remove_child(&mut self) {
        if self.can_remove(self.children - 1) {
            self.children -= 1;
        }
    }
}
