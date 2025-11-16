use sci_fi_food::NameGenerator;

fn main() {
    let mut generator = NameGenerator::new();

    println!("Food combinations:");
    for index in 1..=20 {
        println!("{:02}. {}", index, generator.food_name());
    }

    println!("\nSci-Fi combinations:");
    for index in 1..=24 {
        println!("{:02}. {}", index, generator.scifi_name());
    }
}
