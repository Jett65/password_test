use std::io;

struct Passcode {
    length: u32,
    value: str,
}

impl Passcode {
    fn hash(&self) {
        let split = &self.value[4..];
        for _i in split.chars() {}
    }
}

fn hash(code: &str) -> String {
    let mut pass_list: Vec<u32> = Vec::new();
    let mut pre_list: Vec<u32> = Vec::new();

    for i in code.chars() {
        pass_list.push(i.to_digit(10).unwrap());
    }

    for _i in 0..4 {
        pre_list.push(pass_list[0]);
        pass_list.remove(0);
    }

    // hashes the passcode
    for _i in 0..4 {
        pass_list.push((pass_list[0] * 3) + 4);
        pass_list.remove(0);
    }

    for i in 0..4 {
        pre_list.push(pass_list[i])
    }

    let incode: String = pre_list
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>();

    incode
}
const i: u32 = 1;

fn check(pass: String, compar_pass: String) -> bool {
    let compar_str: &str = &compar_pass[4..];
    let pass_str: &str = &pass[4..];
    let mut stat: bool = false;

    for (i, c) in compar_pass.chars().enumerate() {
        if c == '0' {
            if pass_str == compar_str {
                stat = true;
                break;
            } else {
                stat = false;
            }
        } else {
            if i == 3 {
                stat = false;
                break;
            }
        }
    }
    stat
}

fn main() {
    let passcode = "00001234";
    let enter_pass = "10231234";
    // println!("Enter the passcode");
    //
    // let mut enter_pass = String::new();
    //
    // io::stdin()
    //     .read_line(&mut enter_pass)
    //     .expect("Failed to read line");

    if check(hash(&passcode), hash(&enter_pass)) {
        println!("Access Granted")
    } else {
        println!("Acccess Denied")
    }
}
