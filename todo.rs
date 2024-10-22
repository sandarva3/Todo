use std::io;
fn inc(n: &mut isize){
    *n += 1;
}

fn show(arr: &Vec<&str>, num: &mut isize) {
    for country in arr{
        println!("Country number {}: {}", *num, country);
        inc(num);
    }
}

fn main(){
    println!("HELLO World");
    let a = 4;
    let x = 2;
    println!("The value of 5x2 is: {}", a*x);
    println!("The array: ");
    let arr: [i32; 3] = [9, 5, 3];
    println!("The first value of arr array is: {}", arr[0]);
    for value in arr{
        println!("The value: {}", value);
    }
    //for n in 1..100{
    //    println!("The value of n is: {}", n);
    //}
    let mut countries = vec!["Nepal", "India", "Italy", "France", "USA"];
    let mut num: isize = 1;

    show(&countries, &mut num);
   
    println!("Enter a country name: ");
    let mut country = String::new();
    io::stdin().read_line(&mut country).unwrap();
    println!("The value of country: {}", country.trim());
    countries.insert(0, country.trim());
    println!("The given country name is successfully added to the list");

    num = 1;
    
    show(&countries, &mut num);

}