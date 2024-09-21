fn main() {
    let x: i32 = 1;
    println!("{}", x);
}

fn fib(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;
    if (num == 0) {
        return first;
    }
    if (num == 1) {
        return 1;
    }

    for i in 1..num - 1 {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}
