use std::io;

fn user_temp() -> f32 {
    loop {
        let mut temp: String = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read temp.");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid integer.");
                continue;
            }
        };

        return temp;
    }
}

fn user_unit() -> char {
    loop {
        let mut unit: String = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read temp.");

        let unit: char = match unit.trim().to_lowercase().parse() {
            Ok(char) => char,
            Err(_) => continue,
        };

        if unit == 'f' || unit == 'c' {
            return unit;
        } else {
            continue;
        }
    }
}

fn convert(user_temp: f32, user_unit: char) -> (String, String) {
    if user_unit == 'f' {
        let mut convert_to_c: f32 = user_temp;
        convert_to_c -= 32.0;
        convert_to_c *= 5.0;
        convert_to_c /= 9.0;

        let convert_to_c: String = convert_to_c.to_string();
        let temp_c: String = convert_to_c + "C";

        let user_temp: String = user_temp.to_string();
        let temp_f: String = user_temp + "F";

        return (temp_f, temp_c);
    } else if user_unit == 'c' {
        let mut convert_to_f: f32 = user_temp;
        convert_to_f *= 1.8;
        convert_to_f += 32.0;

        let convert_to_f: String = convert_to_f.to_string();
        let temp_f: String = convert_to_f + "F";

        let user_temp: String = user_temp.to_string();
        let temp_c: String = user_temp + "C";

        return (temp_f, temp_c);
    } else {
        panic!("How tf did you get here")
    };
}

fn main() {
    println!("Please insert the temperature number only:");
    let user_temp = user_temp();
    println!("What is the unit of your initial temp? ");
    let user_unit: char = user_unit();

    let (temp_f, temp_c) = convert(user_temp, user_unit);

    println!("{} = {}", temp_c, temp_f);
}
