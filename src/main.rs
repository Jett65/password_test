// this is a test
fn hash(code: &str) -> String {
    let mut pass_list: Vec<u32> = Vec::new();
    let mut pre_list: Vec<u32> = Vec::new();

    for i in code.chars() {
        pass_list.push(i.to_digit(10).unwrap());
    }

    for i in 0..4 {
        pre_list.push(pass_list[0]);
        pass_list.remove(0);
    }

    // hashes the passcode
    for i in 0..4 {
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

fn check(pass: String, compar_pass: String) -> bool {
    let mut stat: bool = false;

    for (i, c) in compar_pass.chars().enumerate() {
        if c == '0' {
            if compar_pass == pass {
                println!("Accses granted");
                stat = true;
                break;
            }
        } else {
            if i == 3 {
                println!("Invaled Pascode");
                stat = false;
                break;
            }
        }
    }

    println!("{:?}", stat);
    stat
}

fn main() {
    let mut passcode = "10231234";
    let mut user_enter = "10231235";

    check(hash(passcode), hash(user_enter));
}
