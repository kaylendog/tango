use crate::Control;

pub struct ControlServer;

impl Control for ControlServer {
    fn start(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn stop(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn restart(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn config(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn status(&self) -> anyhow::Result<()> {
        todo!()
    }
}
