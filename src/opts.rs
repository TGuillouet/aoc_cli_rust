#[derive(Parser, Debug)]
#[command(author = "Thomas Guillouët", version, about, long_about = None)]
pub struct RawOptions {
    #[arg(short, long)]
    pub day: Option<u32>,

    #[arg(short, long)]
    pub year: Option<i32>,

    #[arg(short, long)]
    pub input_dir: Option<String>,
}
