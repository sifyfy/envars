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

use std::fmt;
use std::collections::BTreeMap;
use std::convert::From;
use std::error::Error;
use std::io;
use std::path;
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

#[derive(Debug)]
pub enum EnvSetError {
    IO(io::Error),
    Yaml(yaml::ScanError),
    ConfigDirIsNotFound,
}

impl fmt::Display for EnvSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            EnvSetError::IO(e) => write!(f, "EnvSetError(IO): {}", e),
            EnvSetError::Yaml(e) => write!(f, "EnvSetError(Yaml): {}", e),
            EnvSetError::ConfigDirIsNotFound => write!(f, "EnvSetError(ConfigDirIsNotFound)"),
        }
    }
}

impl From<io::Error> for EnvSetError {
    fn from(a: io::Error) -> EnvSetError {
        EnvSetError::IO(a)
    }
}

impl From<yaml::ScanError> for EnvSetError {
    fn from(a: yaml::ScanError) -> EnvSetError {
        EnvSetError::Yaml(a)
    }
}

impl Error for EnvSetError {
    fn description(&self) -> &str {
        match *self {
            EnvSetError::IO(e) => e.description(),
            EnvSetError::Yaml(e) => e.description(),
            EnvSetError::ConfigDirIsNotFound => "Config dir is not found.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            EnvSetError::IO(e) => Some(&e),
            EnvSetError::Yaml(e) => Some(&e),
            EnvSetError::ConfigDirIsNotFound => None,
        }
    }
}

/// `EnvSet`のメモリ上での表現型。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvSet {
    name: EnvSetName,
    set: BTreeMap<yaml::Yaml, yaml::Yaml>,
}

impl EnvSet {
    pub fn new(env_set_name: &EnvSetName) -> Result<EnvSet, EnvSetError> {
        Self::read_env_set_yaml(env_set_name).map(|env_set_yaml| {
            EnvSet {
                name: env_set_name.clone(),
                set: env_set_yaml,
            }
        })
    }

    fn read_env_set_yaml(env_set_name: &EnvSetName)
                         -> Result<BTreeMap<yaml::Yaml, yaml::Yaml>, EnvSetError> {
        let content = try!(read_file_content(env_set_name));
        let root = try!(yaml::YamlLoader::load_from_str(&content));
        Ok(root.pop().and_then(|yaml| yaml.as_hash()).map(|o| o.clone()).unwrap_or(BTreeMap::new()))
    }

    fn read_file_content(env_set_name: &EnvSetName) -> Result<String, EnvSetError> {
        let config_dir = try!(s_app_dir::AppDir::new("envars")
                                  .xdg_dir(s_app_dir::XdgDir::Config)
                                  .ok_or(EnvSetError::ConfigDirIsNotFound));
    }
}
