fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // Ok
    let r2 = &s; // Ok
    let r3 = &mut s; // Not ok

    println!("{}, {}, and {}", r1, r2, r3);
}

fn takes_mutable(s: &mut String) {
    s.push_str(" world");
}

fn try_modify(s: &String) {
    s.push_str(" world");
}
