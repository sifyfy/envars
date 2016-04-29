// The MIT License (MIT)
//
// Copyright (c) 2016 Siphilia
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::collections::BTreeMap;
use std::io;
use yaml_rust as yaml;

/// `EnvSet`の名前。`ENV_SET_NAME.yaml`としてファイル名に用いられる。
#[derive(Clone, Debug, AsRef, Eq, PartialEq)]
pub struct EnvSetName(String);

impl EnvSetName {
    /// 半角英数字文字と _ 及び - から成る文字列を引数に与えた場合に`Some(env_set)`が返る。
    /// それ以外の文字列の場合は`None`が返る。
    pub fn new(name: &str) -> Option<EnvSetName> {
        if name.chars()
               .all(|c| c.len_utf8() == 1 && (c.is_alphanumeric() || c == '_' || c == '-')) {
            Some(EnvSetName(name.to_string()))
        } else {
            None
        }
    }
}

pub enum EnvSetError {
    IO(io::Error),
    Yaml(yaml::ScanError),
}

/// `EnvSet`のメモリ上での表現型。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvSet {
    name: EnvSetName,
    set: BTreeMap<yaml::Yaml, yaml::Yaml>,
}

impl EnvSet {
    pub fn new(env_set_name: &EnvSetName) -> Result<EnvSet, EnvSetError> {
        read_env_set_yaml(env_set_name).map(|env_set_yaml| {
            EnvSet {
                name: env_set_name.clone(),
                set: env_set_yaml,
            }
        })
    }

    fn read_env_set_yaml(env_set_name: &env_set_name) -> Result<BTreeMap<yaml::Yaml, yaml::Yaml>, EnvSetError> {
        let result: Result<String, io::Error> = unimplemented!();
        result.map_err(|io_err| EnvSetError::IO(io_err)).and_then(|content| {
            yaml::YamlLoader::load_from_str(&content)
                .map(|vec_yaml| {
                    vec_yaml.pop().and_then(|yaml| yaml.as_hash()).unwrap_or(BTreeMap::new())
                })
                .map_err(|yaml_err| EnvSetError::Yaml(yaml_err))
        })
    }
}
