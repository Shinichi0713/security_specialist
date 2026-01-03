use reqwest;
use scraper::{Html, Selector};
use std::fs;
use std::io::Cursor;
use std::path::Path;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://www.nw-siken.com/kakomon/";
    let client = reqwest::Client::new();

    // 01から07までループ
    for i in 1..=7 {
        let n = format!("{:02}", i);
        let target_url = format!("{}{}_haru/", base_url, n);
        println!("Accessing: {}", target_url);

        // 1. ページのHTMLを取得
        let res = client.get(&target_url).send().await?.text().await?;
        let document = Html::parse_document(&res);
        
        // 2. <a>タグのセレクタを作成
        let selector = Selector::parse("a").unwrap();
        
        // 3. PDFリンクを抽出してダウンロード
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if href.ends_with(".pdf") {
                    // 相対パスを絶対パスに変換（簡易的）
                    let pdf_url = if href.starts_with("http") {
                        href.to_string()
                    } else {
                        format!("{}{}_haru/{}", base_url, n, href)
                    };

                    download_file(&client, &pdf_url).await?;
                }
            }
        }

        // サーバー負荷軽減のため1秒待機
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

async fn download_file(client: &reqwest::Client, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = url.split('/').last().unwrap_or("temp.pdf");
    
    // すでにファイルが存在する場合はスキップ
    if Path::new(filename).exists() {
        println!("Skipping: {} (already exists)", filename);
        return Ok(());
    }

    println!("Downloading: {}", url);
    let response = client.get(url).send().await?;
    
    if response.status().is_success() {
        let mut file = std::fs::File::create(filename)?;
        let mut content = Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
    } else {
        println!("Failed to download: {}", url);
    }

    Ok(())
}