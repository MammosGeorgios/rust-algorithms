mod algorithm;
mod dump;

fn main() {
    println!("Hello algorithms in Rust!");

    let _g = dump::example_unit_test::is_true();

    let sq_one = f64::sqrt(1f64) as usize;
    let sq_two = f64::sqrt(2f64) as usize;
    let sq_three = f64::sqrt(3f64) as usize;
    let sq_four = f64::sqrt(4f64) as usize;
    let sq_120 = f64::sqrt(120f64) as usize;
    let sq_121 = f64::sqrt(121f64) as usize;

    println!("1 : {sq_one} ");
    println!("2 : {sq_two} ");
    println!("3 : {sq_three} ");
    println!("4 : {sq_four} ");
    println!("120 : {sq_120} ");
    println!("121 : {sq_121} ");
}
