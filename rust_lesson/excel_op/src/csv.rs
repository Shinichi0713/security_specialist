use std::fs;
use std::error::Error;
use std::path::Path;
use csv::Writer;
use chrono::{DateTime, Local};

/// ファイルのメタ情報を保持する構造体
/// Serializeを派生させることで、そのままCSVの行として書き出せます
#[derive(serde::Serialize)]
struct FileMeta {
    name: String,
    size_bytes: u64,
    last_modified: String,
    is_dir: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    // 1. 設定：読み込むフォルダと出力先
    let target_dir = "./"; // カレントディレクトリ
    let output_csv = "file_metadata.csv";

    // 2. CSVライターの初期化
    let mut wtr = Writer::from_path(output_csv)?;

    // 3. ディレクトリの読み込み
    let entries = fs::read_dir(target_dir)?;

    println!("Scanning directory: {} ...", target_dir);

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        // ファイル名の取得
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        // 更新日時の変換 (SystemTime -> Readable String)
        let modification_time: DateTime<Local> = metadata.modified()?.into();
        let datetime_str = modification_time.format("%Y-%m-%d %H:%M:%S").to_string();

        // 構造体にデータを格納
        let file_meta = FileMeta {
            name: file_name,
            size_bytes: metadata.len(),
            last_modified: datetime_str,
            is_dir: metadata.is_dir(),
        };

        // 4. CSVへ1行書き出し
        wtr.serialize(file_meta)?;
    }

    // 最後にバッファをフラッシュ
    wtr.flush()?;

    println!("CSV report generated: {}", output_csv);
    Ok(())
}