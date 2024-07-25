pub fn distance_in_2d(self_: Vec<f64>, target: Vec<f64>) -> f64 {
    //given that both represent coords in a 2d system:
    ((self_[0] - target[0]).powf(2.0) + (self_[1] - target[1]).powf(2.0)).sqrt()
}