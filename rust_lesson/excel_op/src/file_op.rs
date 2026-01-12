use std::fs;
use std::path::Path;
use std::io;

fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        // ディレクトリ内のエントリーを取得
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // ディレクトリなら再帰呼び出し
                visit_dirs(&path)?;
            } else {
                // ファイルならパスを表示
                println!("File: {:?}", path);
            }
        }
    }
    Ok(())
}


fn walker() {
    // WalkDir::new("パス") でイテレータを作成
    for entry in WalkDir::new("./")
        .into_iter()
        .filter_map(|e| e.ok()) { // エラー（権限不足など）を無視して展開
            
        println!("{}", entry.path().display());
    }
}

fn main() -> io::Result<()> {
    let start_path = Path::new("./");
    visit_dirs(start_path)?;
    Ok(())
}