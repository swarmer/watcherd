mod errors;
pub use self::errors::*;

use std::process;

use nix::sys;
use nix::sys::wait::WaitStatus;

use super::config;
use super::config::RestartPolicy;


struct State {
    processes: Vec<Process>,
}


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


fn restart_process(process: &mut Process) -> Result<()> {
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


fn handle_exit(state: &mut State, pid: u32, status_code: Option<u8>) -> Result<()> {
    let mut process =
        state.processes.iter_mut()
        .find(|p| p.child.id() == pid)
        .expect("Unexpected child pid");

    let bad_status = match status_code {
        Some(code) => code != 0,
        None => true,
    };

    let needs_restart = match process.task.restart_policy {
        RestartPolicy::Never => false,
        RestartPolicy::Nonzero => bad_status,
        RestartPolicy::Always => true,
    };

    if !needs_restart {
        return Ok(());
    }

    info!("Restarting process with pid {}", pid);
    restart_process(&mut process)
}


fn watch_subprocesses(state: &mut State) -> Result<()> {
    loop {
        let status = try!(sys::wait::wait());
        warn!("Process update: {:?}", status);
        match status {
            WaitStatus::Signaled(pid, _, _) => {
                try!(handle_exit(state, pid as u32, None))
            },
            WaitStatus::Exited(pid, code) => {
                try!(handle_exit(state, pid as u32, Some(code as u8)))
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

    unreachable!();
}
