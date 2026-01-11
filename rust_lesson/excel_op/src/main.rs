use calamine::{Reader, Xlsx, open_workbook, DataType};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. ファイルを開く
    let path = "example.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    // 2. すべてのシート名をループで回す
    for sheet_name in workbook.sheet_names().to_owned() {
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            println!("Sheet: {}", sheet_name);
            
            let mut total_value = 0.0;
            let mut row_count = 0;

            // 3. 各行を読み込む（1行目はヘッダーと想定して skip）
            for row in range.rows().skip(1) {
                // 例えば「2列目 (index 1)」にある数値を集計する場合
                if let Some(cell_value) = row.get(1) {
                    match cell_value {
                        // セルの型をチェックして加算
                        DataType::Float(f) => total_value += f,
                        DataType::Int(i) => total_value += *i as f64,
                        _ => {} // 数値以外は無視
                    }
                }
                row_count += 1;
            }

            println!("  - 総行数: {}", row_count);
            println!("  - 合計値: {:.2}", total_value);
        }
    }

    Ok(())
}