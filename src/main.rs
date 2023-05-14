// Change to string later
fn hash(code: &str) {
    let mut pass_list: Vec<u32> = Vec::new();
    let mut pre_list: Vec<u32> = Vec::new();

    for i in code.chars() {
        pass_list.push(i.to_digit(10).unwrap());
    }
    for i in 0..4 {
        pre_list.push(pass_list[0]);
        pass_list.remove(0);
    }

    // hash the passcode
    for i in 0..4 {
        pass_list.push((pass_list[0] * 3) + 4);
        pass_list.remove(0);
    }

    for i in 0..4 {
        pre_list.push(pass_list[i])
    }
}
fn check(pass: &u32, compar_pass: &u32) -> bool {
    // TODO: find out if there is a 0 in the first 4 numbers
    // let mut char_list = Vec::new();
    // for i in code.chars() {
    //     char_list.push(i.to_digit(10).unwrap())
    // }
    //
    // let mut i = 0;
    // while i < 3 {
    //     if char_list[i] == 0 {
    //         println!("{}", true);
    //         break;
    //     }
    //     i += 1;
    //     if i == 3 {
    //         println!("Invaled password")
    //     }
    // }

    if pass == compar_pass {
        return true;
        // TODO: check if the last four numbers hashed are the same
    } else {
        return false;
    }
}

fn main() {
    let mut passcode = "01231234";
    //let mut user_entry = 1234;

    // let hentry = hash(&mut user_entry);
    let hpass = hash(&passcode);

    // if check(&hpass, &hentry) {
    //     println!("Accuses Granted");
    //     println!("Here is the super secret data:");
    //     println!("I am the super secret data");
    // } else {
    //     println!("Accuses denide");
    // }
}
