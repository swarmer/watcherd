mod errors;
pub use self::errors::*;

use std::fs;

use serde_json;


#[derive(Debug, Clone)]
pub enum RestartPolicy {
    Always,
    Nonzero,
    Never,
}


#[derive(Debug, Clone)]
pub struct Task {
    pub task_name: String,
    pub command_line: String,
    pub restart_policy: RestartPolicy,
}


#[derive(Debug, Clone)]
pub struct Config {
    pub tasks: Vec<Task>,
}


fn read_task(task: &serde_json::Value) -> Result<Task> {
    let map = try!(
        task.as_object()
        .ok_or(Error::Value("Tasks must be objects".to_string()))
    );

    let name = try!(
        map.get("name")
        .and_then(|c| c.as_str())
        .map(|s| s.to_string())
        .ok_or(Error::Value("Tasks must have a string `name`".to_string()))
    );

    let command = try!(
        map.get("command")
        .and_then(|c| c.as_str())
        .map(|s| s.to_string())
        .ok_or(Error::Value("Tasks must have a string `command`".to_string()))
    );

    let restart =
        map.get("restart")
        .and_then(|c| c.as_str())
        .map(str::to_lowercase);

    let restart_policy = try!(match restart.as_ref().map(String::as_ref) {
        Some("always") | None => Ok(RestartPolicy::Always),
        Some("nonzero") => Ok(RestartPolicy::Nonzero),
        Some("never") => Ok(RestartPolicy::Never),
        Some(_) => Err(
            Error::Value(
                "Invalid `restart` value. \
                Must be `always`, `nonzero`, or `never`"
                .to_string()
            )
        )
    });

    Ok(
        Task {
            task_name: name,
            command_line: command,
            restart_policy: restart_policy,
        }
    )
}


fn read_json(json: serde_json::Value) -> Result<Config> {
    let task_objects: &Vec<serde_json::Value> = try!(
        json.find("tasks")
        .and_then(|tasks| tasks.as_array())
        .ok_or(
            Error::Value(
                "Config must by an object with `tasks` array".to_string()
            )
        )
    );

    let mut tasks = Vec::new();
    for task_object in task_objects {
        let task = try!(read_task(task_object));
        tasks.push(task);
    }

    Ok(Config { tasks: tasks })
}


pub fn read_config(config_path: &str) -> Result<Config> {
    let config_file = try!(fs::File::open(config_path));
    let config: serde_json::Value = try!(serde_json::from_reader(config_file));
    read_json(config)
}
