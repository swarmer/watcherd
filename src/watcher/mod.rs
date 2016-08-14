mod errors;
pub use self::errors::*;

use std::process;

use super::config;


struct State {
    processes: Vec<Process>,
}


#[allow(dead_code)]
struct Process {
    task: config::Task,
    child: process::Child,
}


pub fn run(config: config::Config) -> Result<()> {
    debug!("Config: {:?}", config);

    debug!("Starting subprocesses");
    let mut state = State { processes: Vec::new() };
    for task in &config.tasks {
        debug!("Running `{}`", task.command_line);
        let child = try!(
            process::Command::new("sh")
            .arg("-c")
            .arg(&task.command_line)
            .spawn()
        );

        let process = Process { task: task.clone(), child: child };
        state.processes.push(process);
    }
    info!("Started {} processes", state.processes.len());

    for process in &mut state.processes {
        try!(process.child.wait());
    }

    Ok(())
}
