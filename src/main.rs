// use itertools::Itertools;

fn main() {
    let foo = "DamonRolfs".to_string();
    println!("drop_right:{}", foo.get(..(foo.len() - 1)).unwrap());

    // println!("foo:{}", foo);
    // for f in &foo.chars().rev().chunks(5) {
    //     let foo: Vec<char> = f.collect();
    //     let bar: String = foo.into_iter().collect();
    //     println!("f: {}", bar);
    // }
}
