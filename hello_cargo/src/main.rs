fn main() {
    let mut s1 = String::from("Hello");

    let s2 = takes_and_gives_back(&mut s1);

    println!("{}, {}", s1, s2);
}

fn takes_and_gives_back(s: &mut String) -> usize {
    s.push_str("new string");
    s.len()
}
