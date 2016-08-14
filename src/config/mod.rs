mod errors;
pub use self::errors::*;

use std::fs;
use serde_json;


#[derive(Debug)]
pub struct WatcherdTask {
    task_name: String,
    command_line: String,
}


#[derive(Debug)]
pub struct WatcherdConfig {
    tasks: Vec<WatcherdTask>,
}


fn read_task(task: &serde_json::Value) -> Result<WatcherdTask> {
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

    Ok(WatcherdTask { task_name: name, command_line: command })
}


fn read_json(json: serde_json::Value) -> Result<WatcherdConfig> {
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

    Ok(WatcherdConfig { tasks: tasks })
}


pub fn read_config(config_path: &str) -> Result<WatcherdConfig> {
    let config_file = try!(fs::File::open(config_path));
    let config: serde_json::Value = try!(serde_json::from_reader(config_file));
    read_json(config)
}
