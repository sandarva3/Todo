use std::io::{self, Write};

fn inc(n: &mut isize){
    *n += 1;
}


fn show(arr: &Vec<String>) {
    let mut num = 1;
    println!("Todo list: ");
    for task in arr{
        println!("{}. {}", num, task);
        inc(&mut num);
    }
}


fn finished(arr: &Vec<String>) {
    let mut num = 1;
    println!("Completed tasks: ");
    for task in arr{
        println!("{}. {}", num, task);
        inc(&mut num);
    }
}


fn done(arr: &mut Vec<String>, arr2: &mut Vec<String>) {

    let mut num = 1;
    if arr.len() != 0{
        print!("\nEnter task number you completed: ");
        io::stdout().flush().unwrap();
        
        let mut inputval = String::new();
        io::stdin().read_line(&mut inputval).unwrap();
        
        let trimmed = inputval.trim();
        let done = trimmed.parse::<usize>().unwrap();
        
        if done != 0 && done <= arr.len() + 1{
            let value = &arr[done - 1];
            arr2.insert(0, value.to_string());
            arr.remove(done - 1);
        
            println!("GREAT. KEEP GOING ON. Your Completed tasks till now: ");
            for task in arr2{
            println!("{}. {}", num, task);
            inc(&mut num);
            }
    }
    else{
        println!("");
    }
    }
    else{
        println!("There are no tasks to remove. Please add some tasks first.")
    }
}


fn add(arr: &mut Vec<String>) {
    print!("Enter a new task: ");
    io::stdout().flush().unwrap();

    let mut country = String::new();
    io::stdin().read_line(&mut country).unwrap();

    arr.insert(0, country.trim().to_string());
    println!("The given has been added to the list.");
}


fn delete(arr:&mut Vec<String>){
    print!("Enter a task number to remove: ");
    io::stdout().flush().unwrap();

    let mut removecountry = String::new();
    io::stdin().read_line(&mut removecountry).unwrap();

    let trimmed = removecountry.trim();
    let delete = trimmed.parse::<usize>().unwrap();

    if arr.len() != 0{
        if delete != 0 && delete <= arr.len() + 1{
            let value = &arr[delete - 1];
            println!("The following task has been removed from list: (number {}): {}", delete, value);
            arr.remove(delete - 1);
        }
        else{
            println!("Please enter a valid Input. Or Enter 'HELP' for help.")
        }
    }
    else{
        println!("There are no tasks to remove. Please add some tasks first.")
    }
}


fn main(){

    println!(r#"
.--------------------------------------------------------------------------------------------------------------.
|    _____         _         _ _     _          ____          ____                  _                          |
|   |_   _|__   __| | ___   | (_)___| |_       | __ ) _   _  / ___|  __ _ _ __   __| | __ _ _ ____   ____ _    |
|     | |/ _ \ / _` |/ _ \  | | / __| __|      |  _ \| | | | \___ \ / _` | '_ \ / _` |/ _` | '__\ \ / / _` |   |
|     | | (_) | (_| | (_) | | | \__ \ |_ _     | |_) | |_| |  ___) | (_| | | | | (_| | (_| | |   \ V / (_| |   |
|     |_|\___/ \__,_|\___/  |_|_|___/\__(_)    |____/ \__, | |____/ \__,_|_| |_|\__,_|\__,_|_|    \_/ \__,_|   |
|                                                     |___/                                                    |
'--------------------------------------------------------------------------------------------------------------'
    "#);

    let mut countries = vec![];

    let mut dones = vec![];
    loop {
        print!("\n\nEnter your input: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();
        if trimmed == "list"{
            show(& countries);
        }
        else if trimmed == "done"{
            done(&mut countries ,&mut dones);
        }
        else if trimmed == "finished"{
            finished(&mut dones);
        }
        else if trimmed == "add"{
            add(&mut countries);
        }
        else if trimmed == "remove"{
            delete(&mut countries);
        }
        else if trimmed == "HELP" || trimmed == "help"{
            println!(r#"
            HELP COMMANDS:
            --------------------------------------------------------
            - 'list'       : View all remaining tasks in Todo list.
            - 'done'       : Mark a task as completed.
            - 'finished'   : View all tasks that have been completed.
            - 'add'        : Add a new task in Todo list.
            - 'remove'     : Remove a task from Todo list.
            - 'help'       : Show this help message with available commands.
            - 'exit()'     : Exit the application.
            --------------------------------------------------------
            "#)
        }
        else if trimmed == "exit()"{
            println!(r#"
            --------------------------------------------------
            |                                                |
            |        B Y E .  S E E  Y O U  A G A I N.       |
            |                                                |
            --------------------------------------------------
            "#);
            break;
        }
        else{
            println!("The provided input is not a valid input. Please enter 'HELP' for help.");
        }
    }
}