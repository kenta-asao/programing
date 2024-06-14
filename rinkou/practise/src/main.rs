use std::io;

fn main() {
    let a = 4;
    let mut n = 0;
    loop {
        if n * 2 >= a {
            break;
        }
        n += 1;
    }
    println!("{}", n);
    let var = ["x","y"];
    let mut com = vec![];
    com.push([1,1]);
    com.push([4,4]);
    let domain = 5;
    log_encoding(com,domain);
}

fn log_encoding(com: Vec<[i32; 2]>, domain: i32) {
    let variables = log2(domain);
    
    println!("---------------------------------");
    for n in 0..com.len()+1 {
        let mut arr1 = vec![];
        let mut arr2 = vec![];
        let mut a = com[n][0];
        let mut b = com[n][1];
        for i in 0..variables {
            arr1.push(a%2);
            arr2.push(b%2);
            a = a/2;
            b = b/2;
        }
        
        for j in 0..(variables as usize) {
            if arr1[j] == 1 {
                print!{"-"};
            }
            print!("{} ", j+1)
        }

        for k in 0..(variables as usize) {
            if arr2[k] == 1 {
                print!("-");
            }
            print!("{} ", k+(variables as usize)+1)
        }
        println!("0");
    }
    println!("---------------------------------");
}

fn log2 (num: i32) -> i32{
    let mut n:i32 = 0;
    loop {
        if n * 2 >= num {
            break;
        }
        n += 1;
    }
    return n;
}

