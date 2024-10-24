use std::io;
fn inc(n: &mut isize){
    *n += 1;
}


fn userInput(){
    println!("Enter your input: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("The input is: {}", input);
}


fn show(arr: &Vec<String>) {
    let mut num = 1;
    println!("Countries list: ");
    for country in arr{
        println!("Country number {}: {}", num, country);
        inc(&mut num);
    }
}

fn add(arr: &mut Vec<String>) {
    println!("Enter a country name to add: ");
    let mut country = String::new();
    io::stdin().read_line(&mut country).unwrap();

    arr.insert(0, country.trim().to_string());
    println!("The given country name is successfully added to the list");
}

fn delete(arr:&mut Vec<String>){
    println!("Enter which no. country to remove: ");
    let mut removecountry = String::new();
    io::stdin().read_line(&mut removecountry).unwrap();

    let trimmed = removecountry.trim();
    let remvnum = trimmed.parse::<usize>().unwrap();
    println!("The following country has been removed: (number {}): {}", remvnum, arr[remvnum - 1]);
    arr.remove(remvnum - 1);
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
    let mut countries = vec![
    "Nepal".to_string(),
    "India".to_string(),
    "Italy".to_string(),
    "France".to_string(),
    "USA".to_string()
    ];

    userInput();

    show(&countries);

    add(&mut countries);
   
   /*
    println!("Enter a country name: ");
    let mut country = String::new();
    io::stdin().read_line(&mut country).unwrap();
    println!("The value of country: {}", country.trim());
    countries.insert(0, country.trim());
    println!("The given country name is successfully added to the list");
    */

    show(&countries);

    delete(&mut countries);


    show(&countries);
}