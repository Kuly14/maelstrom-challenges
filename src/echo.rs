#![allow(dead_code)]
use std::io::{StdoutLock, Write};

use crate::{Body, Message, Node};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct EchoNode {
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EchoPayload {
    EchoOk { echo: String },
    Echo { echo: String },
}

impl Node<EchoPayload> for EchoNode {
    fn init(node_id: String, node_ids: Vec<String>) -> Self {
        Self { node_id, node_ids }
    }

    fn step(
        &mut self,
        message: Message<EchoPayload>,
        mut writer: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        match message.body.payload {
            EchoPayload::Echo { echo } => {
                let reply = Message {
                    src: message.dest,
                    dest: message.src,
                    body: Body {
                        in_reply_to: message.body.msg_id,
                        msg_id: message.body.msg_id,
                        payload: EchoPayload::EchoOk { echo },
                    },
                };

                serde_json::to_writer(&mut writer, &reply)?;
                writer.write_all(b"\n")?;
                writer.flush()?;

                return Ok(());
            }
            _ => {}
        }
        Err(anyhow::anyhow!("FAILED"))
    }
}
