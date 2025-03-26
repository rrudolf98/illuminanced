pub struct Smoother {
    alpha: f32,   // Smoothing factor (0.0 - 1.0)
    value: u32,   // Smoothed value
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

    pub fn update(&mut self, new_value: u32) -> u32 {
        if !self.initialized {
            self.value = new_value;
            self.initialized = true;
        } else {
            let previous_value_f32 = self.value as f32;
            let new_value_f32 = new_value as f32;
            let val_to_set = new_value;
            self.value = (self.alpha * new_value_f32 + (1.0 - self.alpha) * previous_value_f32) as u32;
            info!(
                "prev {}, val_to_set {}, result {}",prev, val_to_set, self.value
            );
        }
        self.value
    }
}