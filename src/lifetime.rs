fn main() {

    let mut num: i32 = 4;

    add_one(&mut num);

    println!("{}", num)
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}