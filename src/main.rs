use clap::{Parser, Subcommand};

/// Moan CLI encoding & decoding tool
#[derive(Debug, Parser)]
// include version and author information
#[command(name = "moan", version)]
#[command(about = "Moan CLI encoding & decoding tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Encode passed message
    #[command(arg_required_else_help = true)]
    Encode {
        /// Message to encode
        message: String,
    },
    
    /// Decode passed message
    Decode {
        /// Message to decode
        message: String,
    }
}

fn main() {
    let args = Cli::parse();
    
    match args.command {
        Commands::Encode { message } => {
            let encoder = moan::encoder::Encoder::new();
            let encoded = encoder.encode_letters(message);
            
            println!("Encoded message: {}", encoded);
        }
        Commands::Decode { message } => {
            let decoder = moan::decoder::Decoder::new();
            let decoded = decoder.decode_message(message);
            
            if decoded.is_empty() {
                println!("Message is not decodable, are you sure you're using the MOANING?");
                return;
            }
            
            println!("Decoded message: {}", decoded);
        }
    }
}