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

use s_app_dir::{AppDir, XdgDir};
use std::collections::BTreeMap;
use std::collections::btree_map;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::io::Read;
use std::path;
use yaml_rust as yaml;
use yaml_rust::Yaml;

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

impl AsRef<path::Path> for EnvSetName {
    fn as_ref(&self) -> &path::Path {
        &self.0.as_ref()
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

impl error::Error for EnvSetError {
    fn description(&self) -> &str {
        match *self {
            EnvSetError::IO(e) => e.description(),
            EnvSetError::Yaml(e) => e.description(),
            EnvSetError::ConfigDirIsNotFound => "Config dir is not found.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
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
    set: BTreeMap<String, String>,
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

    pub fn iter(&self) -> EnvSetIterator {
        self.set.iter()
    }

    pub fn env(&self, key: &str) -> Option<&String> {
        self.set.get(key)
    }

    fn read_env_set_yaml(env_set_name: &EnvSetName) -> Result<BTreeMap<String, String>, EnvSetError> {
        let content = try!(Self::read_file_content(&env_set_name));
        let root = try!(yaml::YamlLoader::load_from_str(&content));
        Ok(root.pop()
               .and_then(|yaml| yaml.as_hash())
               .map(|o| Self::make_set(&o))
               .unwrap_or_default())
    }

    fn make_set(orig: &BTreeMap<Yaml, Yaml>) -> BTreeMap<String, String> {
        let mut set = BTreeMap::new();
        for (k, v) in orig.iter() {
            k.as_str()
             .and_then(|k_| v.as_str().and_then(|v_| set.insert(k_.to_owned(), v_.to_owned())));
        }
        set
    }

    fn read_file_content<P: AsRef<path::Path>>(env_set_name: P) -> Result<String, EnvSetError> {
        let yaml_path = try!(AppDir::new("envars")
                                 .xdg_dir(XdgDir::Config)
                                 .ok_or(EnvSetError::ConfigDirIsNotFound))
                            .join(env_set_name)
                            .with_extension("yaml");
        let yaml_file = try!(fs::File::open(&yaml_path));
        let mut buf = String::new();
        try!(yaml_file.read_to_string(&mut buf));
        Ok(buf)
    }
}

pub type EnvSetIterator<'a> = btree_map::Iter<'a, String, String>;

impl<'a> IntoIterator for &'a EnvSet {
    type Item = (&'a String, &'a String);
    type IntoIter = EnvSetIterator<'a>;

    fn into_iter(self) -> EnvSetIterator<'a> {
        self.iter()
    }
}
