use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Value to convert
    #[arg(short, long)]
    value: f64,

    /// Input unit
    #[arg(short, long)]
    input_unit: String,

    /// Output unit
    #[arg(short, long)]
    target_unit: String
}

fn main() {
    let args = Args::parse();

    if args.input_unit == "celsius" && args.target_unit == "fahrenheit"{
        
        println!("{}", args.value * (9.0/5.0) + 32.0);
    }
}
