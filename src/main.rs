use gumdrop::Options;
use std::fs;
use std::path::Path;

#[derive(Options, Debug)]
struct ParseArgs {
    ///输入文件
    #[options(help = "输入要解析的md文件", short = "i", long = "input")]
    input_file: String,

    #[options(short = "o", long = "output", help = "输出文件")]
    output_file: String,

    #[options(short = "g", long = "gfm", help = "格式化")]
    gfm: Option<bool>,
}

fn read_file(path: &str) -> std::io::Result<String> {
    let path = Path::new(path).canonicalize()?; // 转为绝对路径
    let content = fs::read_to_string(path)?; // 读取文件内容
    Ok(content)
}

fn parse_md_text(str: String) -> String {
    markdown::to_html(&str)
}

fn parse_md_text_gfm(str: String) -> String {
    markdown::to_html_with_options(&str, &markdown::Options::gfm()).unwrap()
}

fn main() {
    let args = ParseArgs::parse_args_default_or_exit();
    println!("args: {:?}", args);
    match read_file(&args.input_file) {
        Ok(md_text) => {
            let html = if let Some(gfm) = args.gfm {
                if gfm {
                    parse_md_text_gfm(md_text)
                }else {
                    parse_md_text(md_text)
                }
            } else {
                parse_md_text(md_text)
            };
            fs::write(&args.output_file, html).unwrap();
        }
        Err(e) => {
            println!("读取文件失败{:?}", e)
        }
    }
}
