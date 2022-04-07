use clap::Parser;
use std::borrow::{Borrow, BorrowMut};
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use subprocess::NullFile;

#[derive(Parser)]
struct Args {
    /// File to watch
    pub path: PathBuf,

    /// Command to execute
    pub command: String,
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
    let contents: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::with_capacity(5000)));
    let mut sum: (u128, u128) = (0, 0);

    if args.path.exists() {
        loop {
            let file = OpenOptions::new().read(true).open(&args.path);

            if let Ok(mut f) = file {
                if f.read_to_end(contents.lock().unwrap().borrow_mut()).is_ok() {
                    for &b in contents.lock().unwrap().borrow().iter() {
                        sum.0 += b as u128;
                    }
                } else {
                    panic!("Error. Something happened while reading contents!");
                }

                if sum.0 != sum.1 {
                    printf!("File changed. Executing \"{}\"... ", &args.command);
                    let status = subprocess::Exec::shell(&args.command)
                        .stdin(NullFile)
                        .stdout(NullFile)
                        .stderr(NullFile)
                        .join()
                        .unwrap();

                    if !status.success() {
                        panic!("Something went wrong");
                    }
                    printf!("Ended\n");
                }

                sum = (0, sum.0);
                contents.lock().unwrap().borrow_mut().clear();
            } else {
                file.unwrap();
            }

            sleep(Duration::from_secs_f32(2.5));
        }
    } else {
        panic!("Error. Path prob does not exist!");
    }
}
