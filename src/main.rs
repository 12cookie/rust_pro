use std::process::{Command, Stdio, Child};
use std::path::Path;
use std::time::Duration;
use std::thread;

const NUM_INSTANCES: usize = 20;

fn start_instance(python_bin: &str, script_path: &str, id: usize) -> Child {
    println!("Starting instance {}", id + 1);
    Command::new(python_bin)
        .arg(script_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap_or_else(|_| panic!("Failed to start instance {}", id + 1))
}

fn main() {
    let repo_dir = "/Users/aashutoshchauhan/CodeInPython/helloworld";
    let python_script = "helloWorld.py";
    let python_bin = "python3";

    if Path::new(repo_dir).exists() {
        println!("Fetching latest code from git...");
        let status = Command::new("git")
            .arg("-C")
            .arg(repo_dir)
            .arg("pull")
            .status()
            .expect("Failed to run git pull");

        if !status.success() {
            eprintln!("git pull failed!");
            return;
        }
    }
    else {
        eprintln!("Repo directory does not exist: {}", repo_dir);
        return;
    }

    let script_path = Path::new(repo_dir).join(python_script);
    if !script_path.exists() {
        eprintln!("Python script not found: {:?}", script_path);
        return;
    }

    let script_path_str = script_path.to_str().unwrap().to_string();
    let mut processes: Vec<Option<Child>> = {
        let mut v = Vec::with_capacity(NUM_INSTANCES);
        v.resize_with(NUM_INSTANCES, || None);
        v
    };

    loop {
        for i in 0..NUM_INSTANCES {
            let restart_needed = match &mut processes[i] {
                Some(child) => match child.try_wait() {
                    Ok(Some(status)) => {
                        println!("Instance {} exited with status {:?}", i + 1, status);
                        true
                    }
                    Ok(None) => false,
                    Err(e) => {
                        eprintln!("Error checking instance {}: {}", i + 1, e);
                        true
                    }
                },
                None => true,
            };

            if restart_needed {
                processes[i] = Some(start_instance(python_bin, &script_path_str, i));
            }
        }

        thread::sleep(Duration::from_secs(60));
    }
}
