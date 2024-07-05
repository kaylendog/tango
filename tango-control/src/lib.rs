use anyhow::Result;

mod server;

pub trait Control {
    fn start(&self) -> Result<()>;
    fn stop(&self) -> Result<()>;
    fn restart(&self) -> Result<()>;
    fn config(&self) -> Result<()>;
    fn status(&self) -> Result<()>;
}
