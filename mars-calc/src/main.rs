fn main() {
    let mars_weight = calculates_weight_on_mars(95.00);
    println!("Our Weight on mars {} Kg",mars_weight);
}

fn calculates_weight_on_mars(weight: f32) -> f32{
    (weight/ 9.81) * 3.711
}
