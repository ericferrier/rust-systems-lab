pub struct Stats {
    pub mean: f64,
    pub std: f64,
}

pub struct ClassModel {
    pub label: usize,
    pub prior: f64,
    pub features: Vec<Stats>,
}

pub struct NaiveBayes {
    pub classes: Vec<ClassModel>,
}