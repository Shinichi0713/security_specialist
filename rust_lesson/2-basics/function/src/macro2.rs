use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let filename = "config.toml";

    // File::open は Result<File, io::Error> を返す
    let config_file = File::open(filename);

    let file = match config_file {
        // 成功ケース: ファイルが開けた場合
        Ok(file) => {
            println!("✅ 設定ファイル '{}' を正常に開きました。", filename);
            file
        },
        // 失敗ケース: エラーが発生した場合
        Err(error) => match error.kind() {
            // エラーの種類が 'NotFound' の場合
            ErrorKind::NotFound => {
                println!("⚠️ 設定ファイル '{}' が見つかりませんでした。新しく作成します。", filename);
                
                // 新しいファイルを作成しようと試みる
                File::create(filename).unwrap_or_else(|e| {
                    // もしファイル作成も失敗したら、panic（プログラム終了）
                    panic!("致命的なエラー：ファイルを作成できませんでした: {:?}", e);
                })
            },
            // その他のIOエラーの場合
            other_error => {
                // ファイルを開くときに発生したその他のエラーを出力して panic
                panic!("❌ ファイルを開く際に問題が発生しました: {:?}", other_error);
            }
        },
    };
    
    // ここから先は、確実に 'file' 変数に有効な File ハンドルが入っている
    println!("ファイル処理を継続します...");
    
    // 実際の処理は省略...
    // file.read_to_string(&mut content).unwrap(); 
}