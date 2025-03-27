pub struct Kalman {
    q: f32,
    r: f32,
    value: Option<f32>,
    previous_input: Option<f32>,
    covariance: f32,
    threshold: f32,
    wait_counter: u32,
    wait_limit: u32,
}

impl Kalman {
    pub fn new(q: f32, r: f32, covariance: f32) -> Kalman {
        Kalman {
            q,
            r,
            value: None,
            previous_input: None,
            covariance,
            threshold: 10.0, //abs (current - previous) illuminance
            wait_counter: 0,
            wait_limit: 150,
        }
    }
    pub fn process(&mut self, input: f32) -> f32 {
        match self.value {
            None => {
                self.value = Some(input);
                self.previous_input = Some(input);
                input
            }
            Some(x0) => {
                // filter response inertia - remove sudden and momentary changes
                let delta = (input - self.previous_input.unwrap()).abs(); 
                if delta > self.threshold {
                    if self.wait_counter < self.wait_limit {
                        self.wait_counter += 1;
                        return x0;
                    } else {
                        self.wait_counter = 0;
                    }
                } else {
                    self.wait_counter = 0;
                }
                self.previous_input = Some(input);

                //filtering
                let p0 = self.covariance + self.q;
                let k = p0 / (p0 + self.r);
                let x1 = x0 + k * (input - x0);
                let cov = (1.0 - k) * p0;
                self.value = Some(x1);
                self.covariance = cov;
                x1
            }
        }
    }
}
