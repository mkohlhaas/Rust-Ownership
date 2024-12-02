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

    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");

    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push('!');
    println!("{s}");

    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.is_empty());

    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let first = &name.0;
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);

    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let first: &String = get_first(&name);
    // name.1.push_str(", Esq.");
    println!("{first} {}", name.1);

    let mut name = (45, String::from("Ferris"));
    let first: &String = get_first1(&name);
    // name.1.push_str(", Esq.");
    println!("{first} {}", name.1);

    let mut a = [0, 1, 2, 3];
    let x: &mut i32 = &mut a[1];
    *x += 1;
    println!("{a:?}");

    // let mut a = [0, 1, 2, 3];
    // let x = &mut a[1];
    // let y = &a[2];
    // *x += *y;

    let mut a = [0, 1, 2, 3];
    let (a_l, a_r) = a.split_at_mut(2);
    let x = &mut a_l[1];
    let y = &a_r[0];
    *x += *y;
    println!("{a:?}");

    let mut a = [0, 1, 2, 3];
    let x = &mut a[1] as *mut i32;
    let y = &a[2] as *const i32;
    unsafe {
        // DO NOT DO THIS unless you know what you're doing!
        *x += *y;
    }
    println!("{a:?}");

    let mut point = [0, 1];
    let mut x = point[0];
    let y = &mut point[1];
    x += 1;
    *y += 1;
    println!("{} {}", point[0], point[1]);

    // The string would be freed twice at the end of the program
    // let s = String::from("Hello world");
    // let s_ref = &s;
    // let s2 = *s_ref;
    // println!("{s2}");

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

fn get_first(name: &(String, String)) -> &String {
    &name.0
}

fn get_first1(name: &(i32, String)) -> &String {
    &name.1
}
