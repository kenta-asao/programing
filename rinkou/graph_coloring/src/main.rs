use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let variable: [&str; 2] = ["x", "y"]; //変数
    let com_num: i32 = 3; //組み合わせの数
    let color: i32 = 5; //ドメイン
    let mut arr = vec![];
    let mut vertex = Vec::new();
    let mut vertex_num:i32 = 0;

    let path = "graph_coloring.txt"; // 読み込むファイルのパス

    // ファイルを開く
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // 正規表現パターンを定義
    let re = Regex::new(r"p edge (\d+) (\d+)").unwrap();

    // 各行を読み込み、パターンにマッチするかを確認
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re.captures(&line) {
            vertex_num: i32 = caps[1].parse().unwrap();
            let num2: i32 = caps[2].parse().unwrap();
            println!("Number 1: {}, Number 2: {}", num1, num2);
        }
        if line.starts_with("e ") {
            // "e " を取り除く
            let values = line[2..].trim_end();
            let combination: Vec<i32> = values
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            // 0で終わるリストの場合、0を除く
            if let Some(&0) = combination.last() {
                vertex.push(combination[..combination.len() - 1].to_vec());
            } else {
                vertex.push(combination);
            }
        }
    }


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
    encoding(variable, arr, color);
    //let _ = decryption(encoding, color);    
}

fn encoding(var: [&str; 2], com: Vec<[i32; 2]>, domain: i32) {
    //直接符号化：変数・禁止する節の組み合わせ・ドメインを受ける。
    let path = "encoding.cnf";
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
    clasp();
}
fn clasp() {
    let output_file = File::create("result.txt").expect("Failed to create file");

    let output = Command::new("clasp")
        .stdout(Stdio::from(output_file))
        .args(&["-n", "0", "encoding.cnf"])
        .output()
        .expect("failed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

//復号化
fn decryption(encoding: u32, domain: i32) -> std::io::Result<()> {
    // claspの実行結果ファイルのパス
    let path = "result.txt";

    // ファイルを開く
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // 解のカウンタ
    let mut solution_count = 0;
    let mut clauses = Vec::new();

    // 行を読み込みながら処理
    for line in reader.lines() {
        let line = line?;

        // "Answer:" で始まる行はカウンタをインクリメント
        if line.starts_with("Answer:") {
            solution_count += 1;
            continue;
        }

        // "v " で始まる行を処理
        if line.starts_with("v ") {
            // "v " を取り除く
            let values = line[2..].trim_end();
            let literals: Vec<i32> = values
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            // 0で終わるリストの場合、0を除く
            if let Some(&0) = literals.last() {
                clauses.push(literals[..literals.len() - 1].to_vec());
            } else {
                clauses.push(literals);
            }
        }
    }

    // CNF形式の出力

    println!("解");
    println!("x,y");

    if encoding == 1 {
        for n in 0..clauses.len() {
            let mut x = 0;
            let mut y = 0;
            for i in 0..domain {
                if clauses[n][i as usize] > 0 {
                    x = i;
                }
                if clauses[n][(i as usize) + count_variables(&clauses) / 2] > 0 {
                    y = i;
                }
            }
            println!("{},{}", x, y);
        }
    } else if encoding == 2 {
        for n in 0..clauses.len() {
            let mut x = 0;
            let mut y = 0;
            for i in 0..domain {
                if clauses[n][i as usize] > 0 {
                    x = i;
                }
                if clauses[n][(i as usize) + count_variables(&clauses) / 2] > 0 {
                    y = i;
                }
            }
            println!("{},{}", x, y);
        }
    } else if encoding == 3 {
        for n in 0..clauses.len() {
            let mut x = 0;
            let mut y = 0;
            for i in 0..log2(domain) {
                if clauses[n][i as usize] > 0 {
                    x = x + power(2, i as i32);
                }
                if clauses[n][(i as usize) + count_variables(&clauses) / 2] > 0 {
                    y = y + power(2, i as i32);
                }
            }
            println!("{},{}", x, y);
        }
    } else if encoding == 4 {
        for n in 0..clauses.len() {
            let mut x = 0;
            let mut y = 0;
            for i in 0..log2(domain) {
                if clauses[n][i as usize] > 0 {
                    x = x + power(2, i as i32);
                }
                if clauses[n][(i as usize) + count_variables(&clauses) / 2] > 0 {
                    y = y + power(2, i as i32);
                }
            }
            println!("{},{}", x, y);
        }
    } else {
        for n in 0..clauses.len() {
            let mut x = 0;
            let mut y = 0;
            for i in 0..domain {
                if clauses[n][i as usize] > 0 {
                    x = i;
                }
                if clauses[n][(i as usize) + count_variables(&clauses) / 2] > 0 {
                    y = i;
                }
            }
            println!("{},{}", x, y);
        }
    }

    Ok(())
}

// 変数の数を数える関数
fn count_variables(clauses: &Vec<Vec<i32>>) -> usize {
    let mut variables = std::collections::HashSet::new();
    for clause in clauses {
        for &literal in clause {
            variables.insert(literal.abs());
        }
    }
    variables.len()
}

fn contains<T: PartialEq>(array: &[T], value: T) -> bool {
    for element in array.iter() {
        if *element == value {
            return true;
        }
    }
    false
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
