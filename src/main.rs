use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about = "Universal Converter CLI for Laravel")]
struct Args {
    /// 輸入檔案路徑
    #[arg(short, long)]
    input: String,

    /// 輸出檔案路徑
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    println!("開始轉檔...");
    println!("輸入檔案: {}", args.input);
    println!("輸出檔案: {}", args.output);

    let input_path = Path::new(&args.input);
    let output_path = Path::new(&args.output);

    // 基礎圖片讀取與轉換邏輯
    match image::open(input_path) {
        Ok(img) => {
            match img.save(output_path) {
                Ok(_) => println!("轉檔成功！"),
                Err(e) => eprintln!("儲存輸出檔案失敗: {}", e),
            }
        }
        Err(e) => eprintln!("讀取輸入檔案失敗: {}", e),
    }
}