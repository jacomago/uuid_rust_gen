use std::process::exit;

use clap::Parser;
use uuid::Uuid;

/// Generate some Uuid's
#[derive(Parser)]
struct Args {
    /// The amount of id's to generate
    #[clap(short = 'n', long = "number", default_value_t = 1)]
    number: usize,
    /// The version i.e. v4 to use. v4 is default
    #[clap(short = 'v', long = "version", default_value = "v4")]
    version: String,
}

fn uuid_gen(version: &str) -> Uuid {
    match version {
        "v4" => Uuid::new_v4(),
        _ => {
            eprintln!("Version not currently supported.");
            exit(0);
        }
    }
}

fn generate(args: Args) -> Vec<Uuid> {
    (0..args.number).map(|_| uuid_gen(&args.version)).collect()
}

fn main() {
    let args = Args::parse();
    let output = generate(args);
    output.iter().for_each(|id| println!("{:?}", id))
}

#[test]
fn test_uuid_gen_default() {
    assert_eq!(uuid_gen("v4").get_version_num(), 4)
}

#[test]
fn test_generate_single() {
    assert_eq!(
        generate(Args {
            number: 1,
            version: "v4".to_string()
        })
        .len(),
        1
    )
}

#[test]
fn test_generate_multi() {
    assert_eq!(
        generate(Args {
            number: 10,
            version: "v4".to_string()
        })
        .len(),
        10
    )
}
