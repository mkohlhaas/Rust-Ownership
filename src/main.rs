fn main() {
    println!("Hello, world!");
    let _s = return_a_string();
    let _s = return_a_string1();
    let _s = return_a_string2();

    let mut s = String::new();
    return_a_string3(&mut s);
    println!("{s}");

    let mut v: Vec<&str> = vec!["Ferris", "Jr."];
    println!("{}", stringify_name_with_title(&mut v));

    let mut n = 17;
    let a: &mut i32 = &mut n;
    *a += 1;
    let b: &mut i32 = a;
    // let b = a;
    *b += 1;
    println!("{b}");
    println!("{a}");
}

fn return_a_string() -> String {
    String::from("Hello world")
}

fn return_a_string1() -> &'static str {
    "Hello world"
}

use std::rc::Rc;

fn return_a_string2() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}

fn return_a_string3(output: &mut String) {
    output.replace_range(.., "Hello world");
}

fn stringify_name_with_title(name: &mut Vec<&str>) -> String {
    name.push("Esq.");
    name.join(" ")
}
