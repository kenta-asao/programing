use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn decryption(n:i32, domain:i32) -> io::Result<()> {
    // ファイルのパスを指定
    let path = Path::new("result.txt");

    // ファイルを開く
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Vec<Vec<Vec<i32>>>を用意
    let mut vec_of_vec_of_vecs: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut current_vec_of_vecs: Vec<Vec<i32>> = Vec::new();
    let mut current_vec: Vec<i32> = Vec::new();

    // ファイルの各行を読み取る
    for line in reader.lines() {
        let line = line?; // 行を取得

        if line.starts_with("v ") {
            // 'v 'で始まる行を処理
            let values: Vec<i32> = line.split_whitespace() // 空白で分割
                .filter_map(|v| v.parse::<i32>().ok()) // 文字列を整数に変換
                .filter(|&v| v != 0) // 0を除外
                .collect();

            for value in values {
                current_vec.push(value);

                // 5個ごとにVecに格納してリセット
                if current_vec.len() == (domain as usize) {
                    current_vec_of_vecs.push(current_vec);
                    current_vec = Vec::new();
                }
            }
        } else if line.starts_with("c") {
            // 'c'が来たら次のVec<Vec<i32>>に移行
            if !current_vec.is_empty() {
                current_vec_of_vecs.push(current_vec); // 残っている値を格納
                current_vec = Vec::new(); // 新しいVecに移行
            }
            if !current_vec_of_vecs.is_empty() {
                vec_of_vec_of_vecs.push(current_vec_of_vecs); // 現在のVec<Vec<i32>>を格納
                current_vec_of_vecs = Vec::new(); // 新しいVec<Vec<i32>>を開始
            }
        }
    }

    // 最後のAnswerが残っていれば追加
    if !current_vec.is_empty() {
        current_vec_of_vecs.push(current_vec);
    }
    if !current_vec_of_vecs.is_empty() {
        vec_of_vec_of_vecs.push(current_vec_of_vecs);
    }

    // 結果を確認
    let mut answer:Vec<Vec<Vec<i32>>> = Vec::new();
    let mut answer_temp:Vec<Vec<i32>> = Vec::new();
    let mut temp:Vec<i32> = Vec::new();
    for _l in 0..vec_of_vec_of_vecs.len() {
        for i in 0..n {
            for j in 0..n*n {
                for k in 0..domain {
                    if vec_of_vec_of_vecs[i as usize][j as usize][k as usize] > 0 {
                        temp.push(vec_of_vec_of_vecs[i as usize][j as usize][k as usize]-j*domain);
                    }
                    if temp.len() == (n as usize) {
                        answer_temp.push(temp.clone());
                        temp.clear();
                    }
                }
                if answer_temp.len() == (n as usize) {
                    answer.push(answer_temp.clone());
                    answer_temp.clear();
                }
            }
        }
    }

    for i in 0..answer.len() {
        for j in 0..n {
            println!("{:?}", answer[i][j as usize]);
        }
        println!();
    }

    Ok(())

}