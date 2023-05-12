use std::io::Write;
use std::{collections::HashMap, io::StdoutLock};

use crate::{Body, Message, Node};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BroadcastNode {
    node_id: String,
    node_ids: Vec<String>,
    topology: HashMap<String, Vec<String>>,
    seen: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum BroadcastPayload {
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

impl Node<BroadcastPayload> for BroadcastNode {
    fn init(node_id: String, node_ids: Vec<String>) -> Self {
        Self {
            node_id,
            node_ids,
            seen: Vec::new(),
            topology: HashMap::new(),
        }
    }

    fn step(
        &mut self,
        message: crate::Message<BroadcastPayload>,
        mut writer: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        match message.body.payload {
            BroadcastPayload::Broadcast { message: msg } => {

                if !self.seen.contains(&msg) {
                    for (k, _) in &self.topology {
                        let reply = Message {
                            src: message.dest.clone(),
                            dest: k.to_string(),
                            body: Body {
                                in_reply_to: message.body.msg_id,
                                msg_id: message.body.msg_id,
                                payload: BroadcastPayload::Broadcast { message: msg },
                            },
                        };

                        write_message(reply, &mut writer)?;
                    }
                    self.seen.push(msg);
                }

                let reply = Message {
                    src: message.dest.clone(),
                    dest: message.src,
                    body: Body {
                        in_reply_to: message.body.msg_id,
                        msg_id: message.body.msg_id,
                        payload: BroadcastPayload::BroadcastOk,
                    },
                };
                write_message(reply, &mut writer)?;

                return Ok(());
            }

            BroadcastPayload::Read => {
                let reply = Message {
                    src: message.dest,
                    dest: message.src,
                    body: Body {
                        in_reply_to: message.body.msg_id,
                        msg_id: message.body.msg_id,
                        payload: BroadcastPayload::ReadOk {
                            messages: self.seen.clone(),
                        },
                    },
                };

                write_message(reply, &mut writer)?;

                return Ok(());
            }

            BroadcastPayload::Topology { topology } => {
                self.topology = topology;
                let reply = Message {
                    src: message.dest,
                    dest: message.src,
                    body: Body {
                        in_reply_to: message.body.msg_id,
                        msg_id: message.body.msg_id,
                        payload: BroadcastPayload::TopologyOk,
                    },
                };
                write_message(reply, &mut writer)?;
                return Ok(());
            }
            _ => {}
        }

        Err(anyhow::anyhow!("FAILD"))
    }
}

pub fn write_message(
    msg: Message<BroadcastPayload>,
    mut writer: &mut StdoutLock,
) -> anyhow::Result<()> {
    serde_json::to_writer(&mut writer, &msg)?;
    writer.write_all(b"\n")?;
    writer.flush()?;
    Ok(())
}
