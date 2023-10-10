fn main() {
    println!("Hello, world!");
}

fn onea(n: i32, x: i32) -> i32 {
    let mut res = 0;
    for i in 1..n {
        res += x ^ i;
    }
    res
}
fn oneb(n: i32) -> i32 {
    let mut res = 0;
    for i in 1..n {
        res += i ^ i;
    }
    res
}
fn onec(n: i32, x: i32) -> i32 {
    let mut res = 0;
    for i in 1..=n {
        res += (x ^ i) / fak(i);
    }
    res
}

fn fak(i: i32) -> i32 {
    let mut res = 1;
    for i in 1..=i {
        res *= i;
    }
    res
}
fn oned(n: i32, x: i32) -> i32 {
    let mut res = 0;
    for i in 1..n {
        res += -1 ^ (i + 1) * x ^ (i - 1)
    }
    res
}
fn onee(n: i32) -> i32 {
    fak(n - 1)
}
fn onef(n: i32, x: i32) -> i32 {
    let mut res = 1;
    for i in 1..n {
        res *= x ^ i;
    }
    res
}
