use clap::Parser;
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::ops::DerefMut;
use std::thread::sleep;
use std::time::Duration;

use subprocess::*;

mod args;
use args::*;

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
    let mut notified = false;

    let mut proc: RefCell<Popen> = RefCell::new(spawn_process(&args, &mut notified));

    if args.path.exists() {
        let mut first_time = true;

        loop {
            if proc.get_mut().poll().is_some() && !notified {
                printf!("Process ended ");
                notified = true;

                if proc.get_mut().exit_status().unwrap().success() {
                    printf!("successfully!\n");
                } else {
                    printf!("with error!\n");
                }
            }

            let file = OpenOptions::new().read(true).open(&args.path);

            if let Ok(mut f) = file {
                let modified = f.metadata().unwrap().modified().unwrap();

                if modified != last_modified {
                    execute_command(args, &mut proc, &mut notified);
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
                        &mut proc,
                        &mut notified,
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

#[allow(clippy::too_many_arguments)]
fn deep_check(
    args: &Args,
    contents: &mut RefCell<Vec<u8>>,
    sum: &mut (u128, u128),
    lens: &mut (usize, usize),
    first_time: &mut bool,
    f: &mut File,
    deep: (bool, bool),
    proc: &mut RefCell<Popen>,
    notify: &mut bool,
) {
    if f.read_to_end(contents.get_mut()).is_ok() {
        let contents = contents.get_mut();
        {
            if deep.0 {
                lens.0 = contents.len();

                if lens.0 != lens.1 && !*first_time {
                    execute_command(args, proc, notify);
                }

                *lens = (0, lens.0);
            }

            if deep.1 {
                contents.iter().for_each(|&n| sum.0 += n as u128);

                if sum.0 != sum.1 {
                    execute_command(args, proc, notify);
                }

                *sum = (0, sum.0);
            }
        }

        contents.clear();
    } else {
        panic!("Error. Something happened while reading contents!");
    }
}

fn execute_command(args: &Args, mut proc: &mut RefCell<Popen>, notify: &mut bool) {
    printf!(
        "File changed. Finishing{}...",
        if args.verbose {
            format!(" \"{}\"", &args.command)
        } else {
            "".to_string()
        }
    );

    let finish = { proc.deref_mut().get_mut().poll().is_some() };

    if !finish {
        proc.deref_mut().get_mut().terminate().unwrap();
    }

    printf!("Rerunning...");
    let new = spawn_process(&args, notify);

    proc.deref_mut().replace_with(|_| new);
    printf!("Spawned!\n")
}

fn spawn_process(args: &&Args, notify: &mut bool) -> Popen {
    *notify = false;

    Popen::create(
        &args.command.split(' ').collect::<Vec<&str>>(),
        PopenConfig {
            detached: true,
            stdout: if args.interactive {
                Redirection::None
            } else {
                Redirection::Pipe
            },
            stdin: if args.interactive {
                Redirection::None
            } else {
                Redirection::Pipe
            },
            stderr: if args.interactive {
                Redirection::None
            } else {
                Redirection::Pipe
            },
            ..Default::default()
        },
    )
    .expect("couldn't spawn child command")
}
