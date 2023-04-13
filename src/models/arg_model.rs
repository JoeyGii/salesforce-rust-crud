use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

pub struct Args {
    #[arg(short, long)]
    pub fields: Vec<String>,
    #[arg(short, long)]
    pub sobj: String,

    #[arg(short, long)]
    pub id: String,
}

#[derive(Parser, Debug)]
pub struct Get {
    #[arg(short, long)]
    pub sobj: String,
    #[arg(short, long)]
    pub name: String,
}
