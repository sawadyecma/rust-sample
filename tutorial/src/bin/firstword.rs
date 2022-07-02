fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main(){
    let mut s = String::from("as dfghjkl");
    let len = first_word(&s);

    println!("first_word is {}",s);

    s.clear();
    println!("first_word is {}",s);

    let out = &s[..len];

    println!("first_word is {}",len);
    println!("first_word is {}",out);

    // println!("len is {}",len)


}