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

use config;
use error::{Error, Result};
use std::collections::BTreeMap;
use std::collections::btree_map;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::ops;
use std::path;
use yaml_rust as yaml;
use yaml_rust::Yaml;

/// `EnvSet`の名前。`ENV_SET_NAME.yaml`としてファイル名に用いられる。
#[derive(Clone, Debug, Eq, PartialEq)]
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

impl ops::Deref for EnvSetName {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<path::Path> for EnvSetName {
    fn as_ref(&self) -> &path::Path {
        &self.0.as_ref()
    }
}

impl AsRef<String> for EnvSetName {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsRef<str> for EnvSetName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<[u8]> for EnvSetName {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}


/// `EnvSet`のメモリ上での表現型。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvSet {
    name: EnvSetName,
    path: path::PathBuf,
    set: BTreeMap<String, String>,
}

/// CRUD操作を次の様に定義する
/// C: ファイルが存在してればその内容を、無ければ空オブジェクトで作成
/// R: 変数名をキーに値を読み取る関数、変数名のリストは`Iterator`で取得
/// U: 編集はファイルをエディタで開いて行うので、ファイル名を指した状態の`&Path`を返す。
/// D: 紐づいているファイルを削除する
///
/// オブジェクトのファイルへの保存: `write_to_file`関数を用意し、その実行でファイルに書き出す。
impl EnvSet {
    pub fn new(env_set_name: &EnvSetName) -> Result<EnvSet> {
        let yaml_path = try!(Self::yaml_file_path(&env_set_name));
        Self::read_env_set_yaml(&yaml_path).map(|env_set_yaml| {
            EnvSet {
                name: env_set_name.clone(),
                path: yaml_path,
                set: env_set_yaml,
            }
        })
    }

    pub fn empty(env_set_name: &EnvSetName) -> Result<EnvSet> {
        let path: path::PathBuf = try!(Self::yaml_file_path(&env_set_name));
        Ok(EnvSet {
            name: env_set_name.clone(),
            path: path,
            set: BTreeMap::new(),
        })
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn iter(&self) -> EnvSetIterator {
        self.set.iter()
    }

    pub fn env(&self, key: &str) -> Option<&String> {
        self.set.get(key)
    }

    pub fn set_env(&mut self, key: String, value: String) {
        self.set.insert(key, value);
    }

    pub fn file_path(&self) -> &path::Path {
        self.path.as_path()
    }

    pub fn remove_file(&self) -> io::Result<()> {
        fs::remove_file(self.file_path())
    }

    pub fn write_to_file(&self) -> Result<()> {
        let mut file: fs::File = try!(fs::File::open(self.file_path()));
        let yaml: String = try!(self.make_yaml());
        try!(file.write_all(yaml.as_bytes()));
        try!(file.sync_data());
        Ok(())
    }

    pub fn does_env_set_exists(env_set_name: &EnvSetName) -> Result<bool> {
        let path: path::PathBuf = try!(EnvSet::yaml_file_path(&env_set_name));
        match fs::metadata(&path) {
            Ok(_) => Ok(true),
            Err(error) => {
                match error.kind() {
                    io::ErrorKind::NotFound => Ok(false),
                    _ => Err(From::from(error)),
                }
            }
        }
    }

    pub fn yaml_file_path<P: AsRef<path::Path>>(env_set_name: P) -> Result<path::PathBuf> {
        config::config_dir()
            .map(|path| path.join(env_set_name).with_extension("yaml"))
    }

    fn make_yaml(&self) -> Result<String> {
        let mut buf = String::new();
        {
            let mut emitter = yaml::YamlEmitter::new(&mut buf);
            let yaml_hash = yaml::Yaml::Hash(self.set
                                                 .iter()
                                                 .map(|(k, v)| (yaml_str(k), yaml_str(v)))
                                                 .collect());
            try!(emitter.dump(&yaml_hash).map_err(Error::EmitYaml));
        }
        Ok(buf.clone())
    }

    fn read_env_set_yaml<P: AsRef<path::Path>>(yaml_path: P) -> Result<BTreeMap<String, String>> {
        let content: String = try!(Self::read_file_content(yaml_path));
        let root: Vec<yaml::Yaml> = try!(yaml::YamlLoader::load_from_str(&content));
        Ok(root.first()
               .and_then(|yaml| {
                   yaml.as_hash()
                       .map(|o| Self::make_set(&o))
               })
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

    fn read_file_content<P: AsRef<path::Path>>(yaml_path: P) -> Result<String> {
        let mut yaml_file = try!(fs::File::open(&yaml_path));
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

fn yaml_str(s: &str) -> yaml::Yaml {
    yaml::Yaml::String(s.to_owned())
}
