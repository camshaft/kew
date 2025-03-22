pub trait Input {
    type Output;

    fn default_value(&self) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct Range {
    pub step: f32,
    pub value: f32,
    pub range: core::ops::Range<f32>,
}

impl Default for Range {
    fn default() -> Self {
        Self {
            step: 0.0,
            value: 0.0,
            range: 0.0..1.0,
        }
    }
}

impl Input for Range {
    type Output = f32;

    fn default_value(&self) -> Self::Output {
        self.value
    }
}
