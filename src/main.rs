use trapping_rain_water::trap;

fn main() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    println!("Trapped water: {}", trap(height));
}