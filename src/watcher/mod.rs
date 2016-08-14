mod errors;
pub use self::errors::*;

use std::process;

use nix::sys;
use nix::sys::wait::WaitStatus;

use super::config;


struct State {
    processes: Vec<Process>,
}


#[allow(dead_code)]
struct Process {
    task: config::Task,
    child: process::Child,
}


fn start_subprocesses(config: &config::Config) -> Result<State> {
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

    Ok(state)
}


fn restart_process(state: &mut State, pid: u32) -> Result<()> {
    let process =
        state.processes.iter_mut()
        .find(|p| p.child.id() == pid)
        .expect("Unexpected child pid");

    debug!("Running `{}`", process.task.command_line);
    let child = try!(
        process::Command::new("sh")
        .arg("-c")
        .arg(&process.task.command_line)
        .spawn()
    );
    process.child = child;

    Ok(())
}


fn watch_subprocesses(state: &mut State) -> Result<()> {
    loop {
        let status = try!(sys::wait::wait());
        warn!("Process update: {:?}", status);
        match status {
            WaitStatus::Signaled(pid, _, _) | WaitStatus::Exited(pid, _) => {
                info!("Restarting process with pid {}", pid);
                try!(restart_process(state, pid as u32));
            },
            _ => {},
        }
    }
}


pub fn run(config: config::Config) -> Result<()> {
    debug!("Config: {:?}", config);

    debug!("Starting subprocesses");
    let mut state = try!(start_subprocesses(&config));
    info!("Started {} processes", state.processes.len());

    try!(watch_subprocesses(&mut state));

    Ok(())
}
