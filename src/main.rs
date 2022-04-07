use clap::Parser;
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use subprocess::NullFile;

#[derive(Parser)]
#[clap(version, about = "file-watcher")]
struct Args {
    /// File to watch
    pub path: PathBuf,

    /// Command to execute
    pub command: String,

    /// Check interval time
    #[clap(short, long, default_value = "2.5")]
    pub time: f32,

    /// Whether a deep check must be made by contents length
    #[clap(short, long)]
    pub length: bool,

    /// Whether a deep check must be made by contents sum
    #[clap(short, long)]
    pub sum: bool,

    /// Show command
    #[clap(short, long)]
    pub verbose: bool,

    /// Ignore errors when executing comman and don't panic
    #[clap(short, long)]
    pub ignore_errors: bool,
}

macro_rules! printf {
    ( $($t:tt)* ) => {
        {
            use std::io::Write;
            let mut h = std::io::stdout();
            write!(h, $($t)* ).unwrap();
            h.flush().unwrap();
        }
    }
}

fn main() {
    let args = Args::parse();

    run(&args);
}

fn run(args: &Args) -> ! {
    let mut contents: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(10000));
    let mut sum: (u128, u128) = (0, 0);
    let mut lens: (usize, usize) = (0, 0);
    let mut last_modified: std::time::SystemTime = std::time::SystemTime::now();

    if args.path.exists() {
        let mut first_time = true;
        loop {
            let file = OpenOptions::new().read(true).open(&args.path);

            if let Ok(mut f) = file {
                let modified = f.metadata().unwrap().modified().unwrap();

                if modified != last_modified {
                    execute_command(args);
                }

                last_modified = modified;

                if args.length || args.sum {
                    deep_check(
                        args,
                        &mut contents,
                        &mut sum,
                        &mut lens,
                        &mut first_time,
                        &mut f,
                        (args.length, args.sum),
                    );
                }
            } else {
                file.unwrap();
            }

            if first_time {
                first_time = false;
            }
            sleep(Duration::from_secs_f32(args.time));
        }
    } else {
        panic!("Error. Path prob does not exist!");
    }
}

fn deep_check(
    args: &Args,
    contents: &mut RefCell<Vec<u8>>,
    sum: &mut (u128, u128),
    lens: &mut (usize, usize),
    first_time: &mut bool,
    f: &mut File,
    deep: (bool, bool),
) {
    if f.read_to_end(contents.get_mut()).is_ok() {
        let contents = contents.get_mut();
        {
            if deep.0 {
                lens.0 = contents.len();

                if lens.0 != lens.1 && !*first_time {
                    execute_command(args);
                }

                *lens = (0, lens.0);
            }

            if deep.1 {
                contents.iter().for_each(|&n| sum.0 += n as u128);

                if sum.0 != sum.1 {
                    execute_command(args);
                }

                *sum = (0, sum.0);
            }
        }

        contents.clear();
    } else {
        panic!("Error. Something happened while reading contents!");
    }
}

fn execute_command(args: &Args) {
    printf!(
        "File changed. Executing{}...",
        if args.verbose {
            format!(" \"{}\"", &args.command)
        } else {
            "".to_string()
        }
    );

    let status = subprocess::Exec::shell(&args.command)
        .stdin(NullFile)
        .stdout(NullFile)
        .stderr(NullFile)
        .join()
        .unwrap();

    if !status.success() {
        if args.ignore_errors {
            printf!("Error!\n");
        } else {
            panic!("Something went wrong");
        }
    } else {
        printf!("Success!\n");
    }
}
