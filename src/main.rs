use std::io;

fn celsius_to_fahrenheit() {
    loop {
        println!("Enter the number of degrees Celsius you want to convert:");
        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line!");

        let value: f64 = match value.trim().parse() {
            Ok(val) => val,
            Err(_) => { println!("Please enter a valid number!");
                        continue; }
        };

        let fah: f64 = value * (9.0/5.0) + 32.0;
        println!("Converted value in Fahrenheit: {fah}");
        break;
    }
}

fn fahrenheit_to_celsius() {
    loop {
        println!("Enter the number of degrees Fahrenheit you want to convert:");
        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line!");

        let value: f64 = match value.trim().parse() {
            Ok(val) => val,
            Err(_) => { println!("Please enter a valid number!");
                        continue; }
        };

        let cel: f64 = (value - 32.0) * (5.0/9.0);
        println!("Converted value in Fahrenheit: {cel}");
        break;
    }
}

fn choose_conv_type() {
    loop {
        println!("Do you want to convert from Fahrenheit to Celsius(ftc) or Celsius to Fahrenheit(ctf)?");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line!");
        if choice.trim() == "ftc" {
            fahrenheit_to_celsius();
            break;
        } else if choice.trim() == "ctf" {
            celsius_to_fahrenheit();
            break;
        } else {
            println!("Wrong input!");
            continue;
        }
    }
}

fn menu() {
    'outer_loop: loop {
        choose_conv_type();
        'choice_loop: loop {
            println!("Do you want to continue?(y/n)");
            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line!");
        
            if choice.trim() == "y" {
                continue 'outer_loop;
            } else if choice.trim() == "n" {
                break 'outer_loop;
            } else {
                println!("Please enter y or n!");
                continue 'choice_loop;
            }
        }
    }
}

fn main() {
    menu();
}
