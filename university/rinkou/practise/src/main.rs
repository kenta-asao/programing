use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let path = "graph_coloring.txt"; // 読み込むファイルのパス

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
            let u: usize = caps[1].parse().unwrap();
            let v: usize = caps[2].parse().unwrap();
            graph.push([u,v]);
        }
    }    

    Ok(())
}
