pub struct Smoother {
    alpha: f32,   // Smoothing factor (0.0 - 1.0)
    value: f32,   // Smoothed value
    initialized: bool, // To handle first value
}

impl Smoother {
    pub fn new(alpha: f32) -> Self {
        Self {
            alpha,
            value: 0.0,
            initialized: false,
        }
    }

    pub fn update(&mut self, new_value: f32) -> f32 {
        if !self.initialized {
            self.value = new_value;
            self.initialized = true;
        } else {
            self.value = self.alpha * new_value + (1.0 - self.alpha) * self.value;
        }
        self.value
    }
}