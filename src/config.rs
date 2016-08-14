use std;
use std::io;
use std::fmt;
use std::fs;
use serde_json;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    Value(String),
}

pub type Result<T> = std::result::Result<T, self::Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Json(ref err) => err.fmt(f),
            Error::Value(ref string) => string.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::Value(ref string) => string,
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            Error::Value(..) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}


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
