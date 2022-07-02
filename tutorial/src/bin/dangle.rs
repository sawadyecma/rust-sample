fn main() {
    // let reference_to_nothing = dangle();
    let reference = no_dangle();
    println!("reference is {}",reference)
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
