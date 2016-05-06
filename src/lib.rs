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

#![cfg_attr(any(feature="clippy", feature="sorty"), feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]

#![cfg_attr(feature="sorty", plugin(sorty))]
#![cfg_attr(feature="sorty", warn(unsorted_declarations))]

extern crate s_app_dir;
extern crate yaml_rust;

pub mod cmdargs;
pub mod config;
pub mod envset;
pub mod error;

use cmdargs::CmdArgs;
use envset::{EnvSet, EnvSetName};
use error::{Error, Result};
use std::env;
use std::fs;
use std::path;
use std::process;
use std::process::Command;

pub fn start(mode: &mut CmdArgs) -> Result<()> {
    match *mode {
        CmdArgs::Edit(ref env_set) => edit(env_set),
        CmdArgs::List => list(),
        CmdArgs::New(ref env_set) => new(env_set),
        CmdArgs::Run(ref env_set, ref mut cmd) => run(env_set, cmd),
        _ => help(), // Show help.
    }
}

#[cfg(windows)]
fn default_editor() -> String {
    "notepad".to_owned()
}

#[cfg(not(windows))]
fn default_editor() -> String {
    "vi".to_owned()
}

fn edit(env_set_name: &EnvSetName) -> Result<()> {
    let path: path::PathBuf = try!(EnvSet::yaml_file_path(&env_set_name));
    let editor = env::var("EDITOR").unwrap_or_else(|_| default_editor());

    let mut cmd = Command::new(&editor);
    cmd.arg(&path);
    cmd.stdin(process::Stdio::inherit());
    cmd.stdout(process::Stdio::inherit());
    cmd.stderr(process::Stdio::inherit());

    let mut handle: process::Child = try!(cmd.spawn());
    try!(handle.wait());

    Ok(())
}

fn list() -> Result<()> {
    let config_dir: path::PathBuf = try!(config::config_dir());
    for i in try!(fs::read_dir(&config_dir)) {
        let entry: fs::DirEntry = try!(i);
        if let Some(name) = entry.file_name().to_str() {
            println!("{}", name);
        }
    }
    Ok(())
}

fn new(env_set_name: &EnvSetName) -> Result<()> {
    if try!(EnvSet::does_env_set_exists(&env_set_name)) {
        Err(Error::EnvSetExistsWhenInitializing)
    } else {
        init_env_set_file(&env_set_name)
    }
}

/// 記述方法を例示する内容で `EnvSet` ファイルを初期化する。
fn init_env_set_file(env_set_name: &EnvSetName) -> Result<()> {
    let mut env_set: EnvSet = try!(EnvSet::empty(&env_set_name));
    env_set.set_env("VarName".to_owned(), "VarValue".to_owned());
    try!(env_set.write_to_file());
    Ok(())
}

/// `EnvSet`の読み込み～環境変数の設定～指定コマンドの実行、を行う。
fn run(env_set_name: &EnvSetName, cmd: &mut Command) -> Result<()> {
    let env_set: EnvSet = try!(EnvSet::new(&env_set_name));

    for (k, v) in env_set.iter() {
        cmd.env(k, v);
    }
    cmd.stdin(process::Stdio::inherit());
    cmd.stdout(process::Stdio::inherit());
    cmd.stderr(process::Stdio::inherit());

    let mut handle: process::Child = try!(cmd.spawn());
    let exit_status: process::ExitStatus = try!(handle.wait());

    if exit_status.success() {
        Ok(())
    } else {
        Err(Error::ProcessFail(exit_status))
    }
}

/// MODE別詳細ヘルプを出すか、一気にまとめて書くか
fn help() -> Result<()> {
    println!("Usage: envars MODE [OPTIONS]
MODE:
    * run
    * list
    * new
    * edit
    * help

Usage
    run:    envars run ENV_SET_NAME COMMAND
    list:   envars list
    new:    envars new ENV_SET_NAME
    edit:   envars edit ENV_SET_NAME
    help:   envars help

`edit` mode open the EnvSet file with the editor (defined $EDITOR or %EDITOR%).
");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_anything() {}
}
