use clap::Parser;
use rust_decimal::Decimal;

#[derive(Parser)]
pub struct Cli {
    /// The interest amount to divvy up
    #[arg(short, long)]
    pub interest: Decimal,
}
