pub fn log2(num: i32) -> i32 {
    let mut n: i32 = 0;
    loop {
        if n * 2 >= num {
            break;
        }
        n += 1;
    }
    return n;
}

pub fn root(num: i32) -> i32 {
    let mut n:i32 = 0;

    loop {
        if n*n >= num {
            break;
        }
        n +=1;
    }
    return n;
}

pub fn frac(a: i32, b: i32) -> i32 {
    let mut n:i32 = 0;

    loop {
        if n*b >= a{
            break;
        }
        n +=1;
    }
    return n;
}