mod errors;
pub use self::errors::*;

use super::config;


pub fn run(config: config::WatcherdConfig) -> Result<()> {
    println!("Config: {:?}", config);
    Ok(())
}
