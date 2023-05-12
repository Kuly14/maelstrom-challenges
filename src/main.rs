use anyhow::Result;
// use echo::echo::EchoPayload;
// use echo::unique_id::{IdNode, IdPayload};
use echo::broadcast::{BroadcastNode, BroadcastPayload};

fn main() -> Result<()> {
    echo::run::<BroadcastPayload, BroadcastNode>()?;
    Ok(())
}
