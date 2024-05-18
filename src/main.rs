use std::process::Command;
use std::{env, path::Path, thread, time::Duration};

fn main() {
    let argv = env::args().collect::<Vec<_>>();

    if argv.len() <= 1 {
        help();
        return;
    }

    let dev = argv[1].to_owned();

    let path = Path::new(dev.as_str());

    if !path.exists() {
        eprintln!("Device is not currently present. Exiting.");
        return;
    }

    #[cfg(debug_assertions)]
    println!("Waiting for the file to die...");

    while path.exists() {
        thread::sleep(Duration::from_millis(5));
    }

    #[cfg(debug_assertions)]
    println!("File died, suspending system.");
    let _output =  {
        Command::new("systemctl").arg("suspend")
            .output()
            .expect("failed to execute process")
    };
}

fn help() {
    println!("dead_woman_switch DEVFILE");
    println!();
    println!(" Suspends the OS when a device is unplugged. ");
}
