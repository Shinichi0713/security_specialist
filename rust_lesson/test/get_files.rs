use std::fs;
use std::path::Path;
use std::io;

/// 指定したパス内のファイル・フォルダ名を取得して出力する
fn list_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
    // 1. 指定されたパスの内容を読み込む
    // read_dir はイテレータを返す
    let entries = fs::read_dir(path)?;

    println!("--- Directory Contents ---");

    for entry in entries {
        // 各エントリーも Result 型なのでアンラップ（展開）が必要
        let entry = entry?;
        let path = entry.path();

        // 2. ファイルかディレクトリかを判定
        let file_type = if path.is_dir() {
            "[DIR ]"
        } else {
            "[FILE]"
        };

        // 3. ファイル名を取得して表示
        // path.file_name() は Option を返すので、さらに変換が必要
        if let Some(file_name) = path.file_name() {
            println!("{} {:?}", file_type, file_name);
        }
    }

    Ok(())
}

fn main() {
    // 実行例: カレントディレクトリの内容を表示
    let target_dir = "./";
    
    if let Err(e) = list_contents(target_dir) {
        eprintln!("Error reading directory: {}", e);
    }
}