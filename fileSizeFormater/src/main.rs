use std::env;


enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

#[derive(Debug)]
#[allow(dead_code)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}
impl Sizes {
    fn get_sizes(size_str: &str) -> Sizes{
        let parts: Vec<&str> = size_str.split_whitespace().collect();
        if parts.len() != 2 {
            eprintln!("Invalid size input: {}", size_str);
            std::process::exit(1);
        }

        let value: u64 = parts[0].parse().unwrap_or_else(|_| {
            eprintln!("Invalid value input: {}", parts[0]);
            std::process::exit(1);
        });

        let unit = parts[1].to_lowercase();

        let size = match unit.as_str() {
            "b" | "bytes" => FileSize::Bytes(value),
            "kb" | "kilobytes" => FileSize::Kilobytes(value as f64),
            "mb" | "megabytes" => FileSize::Megabytes(value as f64),
            "gb" | "gigabytes" => FileSize::Gigabytes(value as f64),
            _ => {
                eprintln!("Invalid unit: {}", unit);
                std::process::exit(1);
            }
        };

        Sizes {
            bytes: format!("{} bytes", value),
            kilobytes: format!("{:.2} kilobytes", size.to_kilobytes()),
            megabytes: format!("{:.2} megabytes", size.to_megabytes()),
            gigabytes: format!("{:.2} gigabytes", size.to_gigabytes()),
        }

    }
}

trait ToKilobytes {
    fn to_kilobytes(&self) -> f64;
}

trait ToMegabytes {
    fn to_megabytes(&self) -> f64;
}

trait ToGigabytes {
    fn to_gigabytes(&self) -> f64;
}

impl ToKilobytes for FileSize {
    fn to_kilobytes(&self) -> f64 {
        match self {
            FileSize::Bytes(bytes) => *bytes as f64 / 1000.0,
            FileSize::Kilobytes(kb) => *kb,
            FileSize::Megabytes(mb) => *mb * 1000.0,
            FileSize::Gigabytes(gb) => *gb * 1_000_000.0,
        }
    }
}

impl ToMegabytes for FileSize {
    fn to_megabytes(&self) -> f64 {
        match self {
            FileSize::Bytes(bytes) => *bytes as f64 / 1_000_000.0,
            FileSize::Kilobytes(kb) => *kb / 1000.0,
            FileSize::Megabytes(mb) => *mb,
            FileSize::Gigabytes(gb) => *gb * 1000.0,
        }
    }
}

impl ToGigabytes for FileSize {
    fn to_gigabytes(&self) -> f64 {
        match self {
            FileSize::Bytes(bytes) => *bytes as f64 / 1_000_000_000.0,
            FileSize::Kilobytes(kb) => *kb / 1_000_000.0,
            FileSize::Megabytes(mb) => *mb / 1000.0,
            FileSize::Gigabytes(gb) => *gb,
        }
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run <size>");
        std::process::exit(1);
    }

    let size_str = args[1].as_str();
    let result = Sizes::get_sizes(size_str);
    println!("{:?}", result);
}