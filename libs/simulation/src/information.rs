pub struct Information {
    generation: usize,
}

impl Information {
    pub fn new(generation: usize) -> Self {
        Self { generation }
    }

    pub fn generation(&self) -> usize {
        self.generation
    }
}
