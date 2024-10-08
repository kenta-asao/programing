use itertools::Itertools;
use std::io;

fn main() {
    //変数，違反する組み合わせを自分で入力すれば可能
    //let mut st_v1 = String::new();
    //let mut st_v2 = String::new();
    let mut encoding = String::new();

    /*println!("変数を入力してください。");
    println!("１つ目");
    io::stdin()
        .read_line(&mut st_v1)
        .expect("Failed to read line");
    st_v1 = st_v1.replace("\n", "");
    println!("２つ目");
    io::stdin()
        .read_line(&mut st_v2)
        .expect("Failed to read line"); */

    //let variable: [&str; 2] = [&st_v1, &st_v2];
    let variable: [&str; 2] = ["x","y"];
    /*println!("禁止する節の組み合わせは何個ありますか？");
    let mut st_com_num = String::new();
    io::stdin()
        .read_line(&mut st_com_num)
        .expect("Failed to read line");*/
    
    //禁止する節の組み合わせの数
    let com_num: i32 = 3; //st_com_num.trim().parse().unwrap();

    /*println!("ドメインの範囲を入力してください。");
    let mut st_domain = String::new();
    io::stdin()
        .read_line(&mut st_domain)
        .expect("Failed to read line");*/
    let domain: i32 = 3; //st_domain.trim().parse().unwrap();

    let mut arr = vec![];

    for _n in 0..com_num {
        let mut st_numx = String::new();
        let mut st_numy = String::new();

        println!("組み合わせを入力してください。");
        println!("変数１:");
        io::stdin().read_line(&mut st_numx).unwrap();

        println!("変数２:");
        io::stdin().read_line(&mut st_numy).unwrap();

        let i_numx: i32 = st_numx.trim().parse().unwrap();
        let i_numy: i32 = st_numy.trim().parse().unwrap();

        arr.push([i_numx, i_numy]);
    }
    println!("実行する符号化を選択してください。");
    println!("１：直接符号化");
    println!("２：支持符号化");
    println!("３：対数符号化");
    io::stdin()
        .read_line(&mut encoding)
        .expect("Failed to read line");
    let encoding: u32 = encoding.trim().parse().expect("Please type a number!");

    if encoding == 1 {
        direct_encoding(variable, arr, domain);
    } else if encoding == 2 {
        support_encoding(variable, arr, domain);
    } else {
        log_encoding(arr, domain);
    }
}

fn at_least_one(var: [&str; 2], domain: i32) {
    //at-least-one節
    for n in 0..var.len() {
        let m = n as i32;
        for j in 1..domain + 1 {
            let i = j as i32;
            print! {"{} ", m*domain+i};
        }
        println!("0");
    }
}

fn at_most_one(var: [&str; 2], domain: i32) {
    //at-most-one節
    for n in 0..var.len() {
        let m = n as i32;
        for comb in (m * domain + 1..(m + 1) * domain + 1).combinations(2) {
            println!("-{} -{} 0", comb[0], comb[1]);
        }
    }
}

fn log2(num: i32) -> i32 {
    let mut n: i32 = 0;
    loop {
        if n * 2 >= num {
            break;
        }
        n += 1;
    }
    return n;
}

fn power(num: i32, a: i32) -> i32 {
    //numのa乗を返す
    let mut x = 1;
    for _n in 0..a {
        x = x * num;
    }
    return x;
}

fn direct_encoding(var: [&str; 2], com: Vec<[i32; 2]>, domain: i32) {
    //直接符号化：変数・禁止する節の組み合わせ・ドメインを受ける。
    println!("----------直接符号化----------");
    let var_size = var.len() as i32;
    let com_size = com.len() as i32;
    let variables = var_size * domain;
    let clauses = var_size + domain * (domain - 1) * var_size / 2 + com_size;
    println!("p cnf {} {}", variables, clauses);
    at_least_one(var, domain);
    at_most_one(var, domain);
    for n in 0..com.len() {
        //禁止節
        println!("-{} -{} 0", com[n][0] + 1, com[n][1] + domain + 1);
    }
    println!("------------------------------");
}

fn support_encoding(var: [&str; 2], com: Vec<[i32; 2]>, domain: i32) {
    //支持符号化：変数・禁止する節の組み合わせ・ドメインを受ける。
    println!("----------支持符号化----------");
    let var_size = var.len() as i32;
    let com_size = com.len() as i32;
    let variables = var_size * domain;
    let clauses = var_size + domain * (domain - 1) * var_size / 2 + com_size * 2;
    println!("p cnf {} {}", variables, clauses);
    at_least_one(var, domain);
    at_most_one(var, domain);
    //支持節
    for n in 0..com.len() {
        print!("-{} ", com[n][0] + 1);
        for m in 0..domain {
            if m > com[n][1] || m < com[n][1] {
                print!("{} ", m + 1 + domain);
            }
        }
        println!("0");

        print! {"-{} ", com[n][1] + domain + 1};
        for l in 0..domain {
            if l > com[n][1] || l < com[n][1] {
                print!("{} ", l + 1);
            }
        }
        println!("0");
    }
    println!("------------------------------");
}

fn log_encoding(com: Vec<[i32; 2]>, domain: i32) {
    //対数符号化：禁止する節の組み合わせ・ドメインを受ける。
    let variables = log2(domain);
    let com_size = com.len() as i32;
    let clauses = com_size + (power(2, variables) - domain) * 2;

    println!("----------対数符号化----------");
    println!("p cnf {} {}", variables * 2, clauses);

    // 範囲外の値の禁止
    for l in domain +1..power(2, variables)+1 {
        let mut arr1 = vec![];
        let mut arr2 = vec![];
        let mut a = l;
        let mut b = l;
        for _i in 0..variables {
            arr1.push(a % 2);
            arr2.push(b % 2);
            a = a / 2;
            b = b / 2;
        }

        //変数１の範囲外の禁止
        for j in 0..(variables as usize)  {
            if arr1[j] == 1 {
                print! {"-"};
            }
            print!("{} ", j + 1)
        }
        println!("0");

        //変数２の範囲外の禁止
        for k in 0..(variables as usize)  {
            if arr2[k] == 1 {
                print!("-");
            }
            print!("{} ", k + (variables as usize) + 1)
        }
        println!("0");
    }

    // 矛盾節
    for n in 0..com.len() {
        let mut arr1 = vec![];
        let mut arr2 = vec![];
        let mut a = com[n][0];
        let mut b = com[n][1];

        //変数１・２の対数化
        for _i in 0..variables {
            arr1.push(a % 2);
            arr2.push(b % 2);
            a = a / 2;
            b = b / 2;
        }

        //変数１の矛盾
        for j in 0..(variables as usize) {
            if arr1[j] == 1 {
                print! {"-"};
            }
            print!("{} ", j + 1)
        }
        //変数２の矛盾
        for k in 0..(variables as usize) {
            if arr2[k] == 1 {
                print!("-");
            }
            print!("{} ", k + (variables as usize) + 1)
        }
        println!("0");
    }
    println!("------------------------------");
}
