fn main() {
    println!("Enter your weight in kg: ");
    let mut weight = String::new();
    std::io::stdin().read_line(&mut weight).expect("Failed to read line");
    let weight: f64 = weight.trim().parse().expect("Please type a number!");

    println!("Enter your height in m: ");
    let mut height = String::new();
    std::io::stdin().read_line(&mut height).expect("Failed to read line");
    let height: f64 = height.trim().parse().expect("Please type a number!");

    let bmi = weight / height.powf(2.0);

    println!("Your BMI is {:.2}", bmi);
}
