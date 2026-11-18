use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub servers:  BTreeMap<String, Server>,
    pub commands: BTreeMap<String, RemoteCommand>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub host: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteCommand {
    pub name: String,
    pub desc: String,
    pub cmd:  String,
}
