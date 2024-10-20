fn counter(x: i32) {
    // Loop from 0 to x - 1
    for i in 0..x {
        println!("Count: {i}");
    }
}

fn main() {
    println!("Today is 26 September 2024");
    println!("And this is a simple counter program");

    // Call the counter function with the value 10
    counter(10);

    println!("FINISHED");
}

