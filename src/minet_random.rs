use rand::thread_rng;
use rand::Rng;
use rand_distr::{Normal, Distribution};

/// Generates a random `f32` following a Gaussian distribution truncated to [-1, 1].
pub fn random_weight() -> f32 {
    let mut rng = thread_rng();
    // Define a standard normal distribution (mean = 0, std_dev = 1)
    let normal = Normal::new(0.0, 1.0).unwrap();
    
    loop {
        // Corrected line: removed extra spaces between '&' and 'mut'
        let sample: f32 = normal.sample(&mut rng);
        if sample >= -1.0 && sample <= 1.0 {
            return sample;
        }
        // Otherwise, reject and sample again
    }
}
