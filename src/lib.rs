use std::io::{BufRead, StdoutLock, Write};

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod broadcast;
pub mod echo;
pub mod unique_id;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message<Payload> {
    src: String,
    dest: String,
    body: Body<Payload>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body<Payload> {
    in_reply_to: Option<usize>,
    msg_id: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Init {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

pub trait Node<T: DeserializeOwned> {
    fn step(&mut self, message: Message<T>, writer: &mut StdoutLock) -> Result<()>;
    fn init(node_id: String, node_ids: Vec<String>) -> Self;
}

pub fn run<T: DeserializeOwned + Serialize, S: Node<T>>() -> Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let mut lines = stdin.lines();

    let init = lines.next().unwrap().unwrap();
    let init: Message<Init> = serde_json::from_str(&init)?;

    let mut node;
    match init.body.payload {
        Init::Init { node_id, node_ids } => {
            node = S::init(node_id, node_ids);
        }
        _ => panic!(),
    }

    let init_reply = Message {
        src: init.dest,
        dest: init.src,
        body: Body {
            in_reply_to: init.body.msg_id,
            msg_id: init.body.msg_id,
            payload: Init::InitOk,
        },
    };

    serde_json::to_writer(&mut stdout, &init_reply).context("Failed to write to stdout")?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;

    for line in lines {
        let line = line?;
        let message: Message<T> =
            serde_json::from_str(&line).context("Faield to deserialize from stdin")?;

        node.step(message, &mut stdout)?;
    }

    Ok(())
}
