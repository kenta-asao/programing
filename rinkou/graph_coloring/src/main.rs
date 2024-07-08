use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::process::{Command, Stdio};
use regex::Regex;

fn main() -> io::Result<()> {
    let color: i32 = 4; //ドメイン
    let path = "myciel4.txt"; // 読み込むファイルのパス

    // ファイルを開く
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // 正規表現パターンを定義
    let p_edge_re = Regex::new(r"p edge (\d+) (\d+)").unwrap();
    let e_re = Regex::new(r"e (\d+) (\d+)").unwrap();

    // グラフの頂点とエッジを格納する
    let mut num_vertices = 0;
    let mut edge = 0;
    let mut graph = vec![];

    // 各行を読み込み、パターンにマッチするかを確認
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = p_edge_re.captures(&line) {
            num_vertices = caps[1].parse().unwrap();
            edge = caps[2].parse().unwrap();
            println!("Number of vertices: {}", num_vertices);
        } else if let Some(caps) = e_re.captures(&line) {
            let u: i32 = caps[1].parse().unwrap();
            let v: i32 = caps[2].parse().unwrap();
            graph.push([u,v]);
        }
    }    

    encoding(color, graph, edge, num_vertices);
    //let _ = decryption(color, num_vertices);    
    Ok(())
}

fn encoding(color:i32, graph: Vec<[i32; 2]>, edge: i32, num_vertices: i32) {
    //直接符号化：変数・禁止する節の組み合わせ・ドメインを受ける。
    let path = "encoding.cnf";
    let mut file = File::create(path).expect("file not found.");

    println!("----------直接符号化----------");
    let variables = num_vertices * color;
    let clauses = num_vertices + color * (color - 1) * num_vertices / 2 + edge*color;

    writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
    /*at-least-one */
    for n in 0..num_vertices {
        let m = n as i32;
        for j in 1..color + 1 {
            let i = j as i32;
            write! {file, "{} ", m*color+i}.expect("cannot write.");
        }
        writeln!(file, "0").expect("cannot write.");
    }
    /*at-most-one */
    for n in 0..num_vertices {
        let m = n as i32;
        for comb in (m * color + 1..(m + 1) * color + 1).combinations(2) {
            writeln!(file, "-{} -{} 0", comb[0], comb[1]).expect("cannot write.");
        }
    }

    /*禁止節 */
    for i in 0..(edge as usize) {
        for j in 0..color {
            writeln!(file, "-{} -{} 0", (graph[i][0]-1)*color+(j as i32)+1, (graph[i][1]-1)*color+(j as i32)+1).expect("cannot write.");
        }
    }

    /*claspの実行 */
    clasp();
}

fn log_encoding(color:i32, graph: Vec<[i32; 2]>, edge: i32, num_vertices: i32) {
    let path = "encoding.cnf";
    let mut file = File::create(path).expect("file not found.");

    println!("----------対数符号化----------");
    let variables = log2(color);
    let clauses = edge + (power(2, variables) - color) * num_vertices;

    writeln!(file, "p cnf {} {}", variables*num_vertices, clauses).expect("cannot write.");

    // 範囲外の値の禁止
    for l in color + 1..power(2, variables) + 1 {
        //ドメイン外の値の２進表現
        let mut arr = vec![];
        let mut a = l;
        for _i in 0..variables {
            arr1.push(a % 2);
            a = a / 2;
        }

        //変数の範囲外の禁止
        for k in 0..(variables as usize) {
            for m in 0..num_vertices {
                if arr[k] == 1 {
                    write!(file, "-").expect("cannot write.");
                }
                write!(file, "-{} ", k + (variables as usize) + 1).expect("cannot write.");
            }
            writeln!(file, "0").expect("cannot write.");
        }
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
fn decryption(domain: i32, num_vertices:i32) -> std::io::Result<()> {
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

        for n in 0..clauses.len() {
            for i in 0..domain {
                for j in 0..num_vertices {
                    if clauses[n][(i as usize) + (j as usize)] > 0 {
                        print!("{} ", i*j+1);
                    }
                }
                
            }
            println!("");
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
