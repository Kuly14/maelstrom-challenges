#![allow(dead_code)]
use std::io::{StdoutLock, Write};

use serde::{Deserialize, Serialize};

use crate::{Body, Message, Node};

pub struct IdNode {
    node_id: String,
    node_ids: Vec<String>,
    count: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IdPayload {
    Generate,
    GenerateOk { id: String },
}

impl Node<IdPayload> for IdNode {
    fn init(node_id: String, node_ids: Vec<String>) -> Self {
        Self {
            node_id,
            node_ids,
            count: 1,
        }
    }

    fn step(
        &mut self,
        message: crate::Message<IdPayload>,
        mut writer: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        match message.body.payload {
            IdPayload::Generate => {
                let id = format!("{}-{}", self.count, self.node_id);
                let reply = Message {
                    src: message.dest,
                    dest: message.src,
                    body: Body {
                        in_reply_to: message.body.msg_id,
                        msg_id: message.body.msg_id,
                        payload: IdPayload::GenerateOk { id },
                    },
                };

                self.count += 1;

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
