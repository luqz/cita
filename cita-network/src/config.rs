// Copyright Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cita_types::{clean_0x, Address};
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use util::parse_config;

#[derive(Debug, Deserialize, Clone)]
pub struct NetConfig {
    pub port: Option<usize>,
    pub peers: Option<Vec<PeerConfig>>,
    pub max_connects: Option<usize>,
    pub enable_tls: Option<bool>,
    pub enable_discovery: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PeerConfig {
    pub ip: Option<String>,
    pub port: Option<usize>,
}

impl NetConfig {
    pub fn new(path: &str) -> Self {
        parse_config!(NetConfig, path)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddressConfig {
    pub addr: Address,
}

impl AddressConfig {
    pub fn new(path: &str) -> Self {
        let mut buffer = String::new();
        let addr = match File::open(path).and_then(|mut f| f.read_to_string(&mut buffer)) {
            Ok(_) => Address::from_str(clean_0x(&buffer)).unwrap(),
            Err(_) => {
                info!("[Config] Cannot find address file, using a random Address instead.");
                Address::random()
            }
        };

        AddressConfig { addr }
    }
}

#[cfg(test)]
mod tests {
    use super::NetConfig;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn basic_test() {
        let toml_str = r#"
        port = 4000
        enable_tls = true
        max_connects = 4
        id_card = 9
        [[peers]]
            ip = "127.0.0.1"
            port = 4001
            common_name = "test1.cita"
        [[peers]]
            ip = "127.0.0.1"
            port = 4002
        "#;

        let mut tmp_file: NamedTempFile = NamedTempFile::new().unwrap();
        tmp_file.write_all(toml_str.as_bytes()).unwrap();
        let path = tmp_file.path().to_str().unwrap();
        let config = NetConfig::new(path);

        assert_eq!(config.port, Some(4000));
        assert_eq!(config.max_connects, Some(4));
        assert_eq!(config.enable_tls, Some(true));
        assert_eq!(config.peers.unwrap().len(), 2);
        assert_eq!(config.enable_discovery, None);
    }
}
