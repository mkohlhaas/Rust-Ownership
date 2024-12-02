fn main() {
    let mut n = 17;
    let a: &mut i32 = &mut n;
    *a += 1;
    let b: &mut i32 = a;
    // let b = a;
    *b += 1;
    println!("{b}");
    println!("{a}");

    // The string would be freed twice at the end of the program
    // let s = String::from("Hello world");
    // let s_ref = &s;
    // let s2 = *s_ref;
    // println!("{s2}");

    // rustc: cannot move out of `*v_ref` which is behind a shared reference
    // ⇒ would lead to a double-free
    // let v = vec![1, 2, 3];
    // let v_ref: &Vec<i32> = &v;
    // let v2 = *v_ref;
    // drop(v2);
    // drop(v);

    // basically the same as
    // let v = vec![1, 2, 3];
    // let v2 = v;
    // drop(v2);
    // drop(v);
}
