use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process::Command;

fn main() {
    let mut encoding = String::new();
    let variable: [&str; 2] = ["x", "y"]; //変数
    let com_num: i32 = 3; //組み合わせの数
    let domain: i32 = 3; //ドメイン
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
    println!("４：multivalued_encoding");
    io::stdin()
        .read_line(&mut encoding)
        .expect("Failed to read line");
    let encoding: u32 = encoding.trim().parse().expect("Please type a number!");

    if encoding == 1 {
        direct_encoding(variable, arr, domain);
    } else if encoding == 2 {
        support_encoding(variable, arr, domain);
    } else if encoding == 3 {
        log_encoding(arr, domain);
    } else{
        multivvalued_encoding(variable, arr, domain);
    }
}

/*fn at_least_one(var: [&str; 2], domain: i32) {
    //at-least-one節
    for n in 0..var.len() {
        let m = n as i32;
        for j in 1..domain + 1 {
            let i = j as i32;
            print! {"{} ", m*domain+i};
        }
        println!("0");
    }
}*/

/*fn at_most_one(var: [&str; 2], domain: i32) {
    //at-most-one節
    for n in 0..var.len() {
        let m = n as i32;
        for comb in (m * domain + 1..(m + 1) * domain + 1).combinations(2) {
            println!("-{} -{} 0", comb[0], comb[1]);
        }
    }
}*/

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
    let path = "direct_encoding.cnf";
    let mut file = File::create(path).expect("file not found.");

    println!("----------直接符号化----------");
    let var_size = var.len() as i32;
    let com_size = com.len() as i32;
    let variables = var_size * domain;
    let clauses = var_size + domain * (domain - 1) * var_size / 2 + com_size;
    writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
    /*at-least-one */
    for n in 0..var.len() {
        let m = n as i32;
        for j in 1..domain + 1 {
            let i = j as i32;
            write! {file, "{} ", m*domain+i}.expect("cannot write.");
        }
        writeln!(file, "0").expect("cannot write.");
    }
    /*at-most-one */
    for n in 0..var.len() {
        let m = n as i32;
        for comb in (m * domain + 1..(m + 1) * domain + 1).combinations(2) {
            writeln!(file, "-{} -{} 0", comb[0], comb[1]).expect("cannot write.");
        }
    }

    /*禁止節 */
    for n in 0..com.len() {
        writeln!(file, "-{} -{} 0", com[n][0] + 1, com[n][1] + domain + 1).expect("cannot write.");
    }

    /*claspの実行 */
    let output = Command::new("clasp")
        .args(&["-n", "0", "direct_encoding.cnf"])
        .output()
        .expect("failed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn support_encoding(var: [&str; 2], com: Vec<[i32; 2]>, domain: i32) {
    //支持符号化：変数・禁止する節の組み合わせ・ドメインを受ける。
    let path = "support_encoding.cnf";
    let mut file = File::create(path).expect("file not found.");

    println!("----------支持符号化----------");
    let var_size = var.len() as i32;
    let com_size = com.len() as i32;
    let variables = var_size * domain;
    let clauses = var_size + domain * (domain - 1) * var_size / 2 + com_size * 2;
    writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");

    /*at-least-one */
    for n in 0..var.len() {
        let m = n as i32;
        for j in 1..domain + 1 {
            let i = j as i32;
            write! {file, "{} ", m*domain+i}.expect("cannot write.");
        }
        writeln!(file, "0").expect("cannot write.");
    }

    /*at-most-one */
    for n in 0..var.len() {
        let m = n as i32;
        for comb in (m * domain + 1..(m + 1) * domain + 1).combinations(2) {
            writeln!(file, "-{} -{} 0", comb[0], comb[1]).expect("cannot write.");
        }
    }

    //支持節
    for n in 0..com.len() {
        write!(file, "-{} ", com[n][0] + 1).expect("cannot write.");
        for m in 0..domain {
            if m > com[n][1] || m < com[n][1] {
                write!(file, "{} ", m + 1 + domain).expect("cannot write.");
            }
        }
        writeln!(file, "0").expect("cannot write.");

        write! {file, "-{} ", com[n][1] + domain + 1}.expect("cannot write.");
        for l in 0..domain {
            if l > com[n][1] || l < com[n][1] {
                write!(file, "{} ", l + 1).expect("cannot write.");
            }
        }
        writeln!(file, "0").expect("cannot write.");
    }

    /*claspの実行 */
    let output = Command::new("clasp")
        .args(&["-n", "0", "support_encoding.cnf"])
        .output()
        .expect("failed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn log_encoding(com: Vec<[i32; 2]>, domain: i32) {
    //対数符号化：禁止する節の組み合わせ・ドメインを受ける。
    let path = "log_encoding.cnf";
    let mut file = File::create(path).expect("file not found.");

    let variables = log2(domain);
    let com_size = com.len() as i32;
    let clauses = com_size + (power(2, variables) - domain) * 2;

    println!("----------対数符号化----------");
    writeln!(file, "p cnf {} {}", variables * 2, clauses).expect("cannot write.");

    // 範囲外の値の禁止
    for l in domain + 1..power(2, variables) + 1 {
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
        for j in 0..(variables as usize) {
            if arr1[j] == 1 {
                write! {file, "-"}.expect("cannot write.");
            }
            write!(file, "-{} ", j + 1).expect("cannot write.");
        }
        writeln!(file, "0").expect("cannot write.");

        //変数２の範囲外の禁止
        for k in 0..(variables as usize) {
            if arr2[k] == 1 {
                write!(file, "-").expect("cannot write.");
            }
            write!(file, "-{} ", k + (variables as usize) + 1).expect("cannot write.");
        }
        writeln!(file, "0").expect("cannot write.");
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
                write! {file, "-"}.expect("cannot write.");
            }
            write!(file, "{} ", j + 1).expect("cannot write.");
        }
        //変数２の矛盾
        for k in 0..(variables as usize) {
            if arr2[k] == 1 {
                write!(file, "-").expect("cannot write.");
            }
            write!(file, "{} ", k + (variables as usize) + 1).expect("cannot write.");
        }
        writeln!(file, "0").expect("cannot write.");
    }
    /*claspの実行 */
    let output = Command::new("clasp")
        .args(&["-n", "0", "log_encoding.cnf"])
        .output()
        .expect("failed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn log_support_encoding(com: Vec<[i32; 2]>, domain: i32) {
    //対数符号化：禁止する節の組み合わせ・ドメインを受ける。
    let path = "log_support_encoding.cnf";
    let mut file = File::create(path).expect("file not found.");

    let variables = log2(domain);
    let com_size = com.len() as i32;
    let clauses = com_size + (power(2, variables) - domain) * 2;

    println!("----------対数支持符号化----------");
    writeln!(file, "p cnf {} {}", variables * 2, clauses).expect("file not found.");

    // 範囲外の値の禁止
    for l in domain + 1..power(2, variables) + 1 {
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
        for j in 0..(variables as usize) {
            if arr1[j] == 1 {
                write! {file, "-"}.expect("file not found.");
            }
            write!(file, "-{} ", j + 1).expect("file not found.");
        }
        writeln!(file, "0").expect("file not found.");

        //変数２の範囲外の禁止
        for k in 0..(variables as usize) {
            if arr2[k] == 1 {
                write!(file, "-").expect("file not found.");
            }
            write!(file, "-{} ", k + (variables as usize) + 1).expect("file not found.");
        }
        writeln!(file, "0").expect("file not found.");
    }

    //支持節　未完成
    for n in 0..com.len() {
        write!(file, "-{} ", com[n][0] + 1).expect("cannot write.");
        for m in 0..domain {
            if m > com[n][1] || m < com[n][1] {
                write!(file, "{} ", m + 1 + domain).expect("cannot write.");
            }
        }
        writeln!(file, "0").expect("cannot write.");

        write! {file, "-{} ", com[n][1] + domain + 1}.expect("cannot write.");
        for l in 0..domain {
            if l > com[n][1] || l < com[n][1] {
                write!(file, "{} ", l + 1).expect("cannot write.");
            }
        }
        writeln!(file, "0").expect("cannot write.");
    }

    /*claspの実行 */
    let output = Command::new("clasp")
        .args(&["-n", "0", "log_support_encoding.cnf"])
        .output()
        .expect("failed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn multivvalued_encoding(var: [&str; 2], com: Vec<[i32; 2]>, domain: i32) {
    //直接符号化：変数・禁止する節の組み合わせ・ドメインを受ける。
    let path = "multivvalued_encoding.cnf";
    let mut file = File::create(path).expect("file not found.");

    println!("----------multivvalued_encoding----------");
    let var_size = var.len() as i32;
    let com_size = com.len() as i32;
    let variables = var_size * domain;
    let clauses = var_size + domain * (domain - 1) * var_size / 2 + com_size;
    writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
    /*at-least-one */
    for n in 0..var.len() {
        let m = n as i32;
        for j in 1..domain + 1 {
            let i = j as i32;
            write! {file, "{} ", m*domain+i}.expect("cannot write.");
        }
        writeln!(file, "0").expect("cannot write.");
    }

    /*禁止節 */
    for n in 0..com.len() {
        writeln!(file, "-{} -{} 0", com[n][0] + 1, com[n][1] + domain + 1).expect("cannot write.");
    }

    /*claspの実行 */
    let output = Command::new("clasp")
        .args(&["-n", "0", "multivvalued_encoding.cnf"])
        .output()
        .expect("failed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
