#[inline]
pub fn relu(x: f32) -> f32 {
    if x > 0.0 { x } else { 0.0 }
}

#[inline]
pub fn leaky_relu(x: f32, alpha: f32) -> f32 {
    if x > 0.0 { x } else { alpha * x }
}

// Usage with default alpha = 0.01
pub fn leaky_relu_default(x: f32) -> f32 {
    leaky_relu(x, 0.01)
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

pub fn tanh(x: f32) -> f32 {
    x.tanh()
}