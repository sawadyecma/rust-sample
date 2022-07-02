fn main(){

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える

    println!("{}", s); // これは`hello, world!`と出力する

    drop(s);

    let s = String::from("hello");

    println!("{}", s); // これは`hello, world!`と出力する



    /*
    let mut s = "hello";

    s +=", world!";

    println!("{}", s); // これは`hello, world!`と出力する
     */

    let mut x = 5;

    let y = x;


    x += 1;



    println!("x is {}",x);
    println!("y is {}",y);

    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    s2.push_str("111");

    println!("s1 is {}",s1);
    println!("s2 is {}",s2);


    /* 
    let x = 5;

    let y = x.copy();


    x += 1;



    println!("x is {}",x);
    println!("y is {}",y);
    */

    println!("=======================");
    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない
    // println!("s: {}",s);

    let x = 5;                      // xがスコープに入る

    makes_copy(x);

    println!("x: {}",x);
}

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} 