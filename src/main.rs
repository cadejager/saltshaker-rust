use clap::Parser;
use std::thread;
use std::time::SystemTime;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// Number of threads to use
    #[arg(long, default_value_t = 4)]
    threads: u16,

    /// Time to run in seconds
    #[arg(short, long, default_value_t = 60)]
    time: u64,
}

fn main() {
    let args = Args::parse();
    let mut handles = Vec::new();

    for _ in 0..args.threads {
        let handle = thread::spawn(move || {
            let start_time = SystemTime::now();

            let mut last = rand::random::<f64>();
            loop {
                for _ in 0..4096 { 
                    let cur = rand::random::<f64>();
                    let _plus = last+cur;
                    let _dif1 = last-cur;
                    let _dif2 = cur-last;
                    let _mult = cur*last;
                    let _div1 = cur/last;
                    let _div2 = last/cur;
                    let _mod1 = last%cur;
                    let _mod2 = cur%last;
                    last = cur;
                }

                match SystemTime::now().duration_since(start_time) {
                    Ok(duration) => {
                        if args.time <= duration.as_secs() {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {:#?}", e);
                        std::process::exit(1);
                    }
                }

            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

}
