use std::io;
use rand::Rng;
use clap::Parser;

/// Simple random number generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short,long)]
	bottom: u8,

	#[arg(short,long)]
	top: u8,
}

fn main(){
	// ヴァリアブル作成
	let args = Args::parse();
	let mut min = args.bottom;
	let mut max = args.top;
	let mut stdout = io::stdout();
	let stdin = io::stdin();
	
	// ランダム整数の印刷する
	let random_number = rand::thread_rng().gen_range(min..=max);
	println!("Random number: {random_number}");

}
