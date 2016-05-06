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
pub mod envset;

use cmdargs::CmdArgs;
use envset::{EnvSet, EnvSetError, EnvSetName};
use std::error::Error;
use std::fmt;
use std::io;
use std::process;
use std::process::Command;
use std::result;

#[derive(Debug)]
pub enum EnvarsError {
    EnvSet(EnvSetError),
    IO(io::Error),
    ProcessFail(process::ExitStatus),
}

impl fmt::Display for EnvarsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnvarsError::EnvSet(ref e) => write!(f, "EnvarsError(EnvSetError): {}", e),
            EnvarsError::IO(ref e) => write!(f, "EnvarsError(IO): {}", e),
            EnvarsError::ProcessFail(ref status) => {
                write!(f, "EnvarsError(ProcessFail): {}", status)
            }
        }
    }
}

impl From<io::Error> for EnvarsError {
    fn from(e: io::Error) -> EnvarsError {
        EnvarsError::IO(e)
    }
}

impl From<EnvSetError> for EnvarsError {
    fn from(e: EnvSetError) -> EnvarsError {
        EnvarsError::EnvSet(e)
    }
}

impl Error for EnvarsError {
    fn description(&self) -> &str {
        match *self {
            EnvarsError::EnvSet(ref e) => e.description(),
            EnvarsError::IO(ref e) => e.description(),
            EnvarsError::ProcessFail(_) => "Running child process was fail.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            EnvarsError::EnvSet(ref e) => Some(e),
            EnvarsError::IO(ref e) => Some(e),
            EnvarsError::ProcessFail(_) => None,
        }
    }
}

pub type Result = result::Result<(), EnvarsError>;

pub fn start(mode: &mut CmdArgs) -> Result {
    match *mode {
        CmdArgs::Edit(ref env_set) => edit(env_set),
        CmdArgs::List => list(),
        CmdArgs::New(ref env_set) => new(env_set),
        CmdArgs::Run(ref env_set, ref mut cmd) => run(env_set, cmd),
        _ => help(), // Show help.
    }
}

fn edit(env_set_name: &EnvSetName) -> Result {
    unimplemented!();
}

fn list() -> Result {
    unimplemented!();
}

fn new(env_set_name: &EnvSetName) -> Result {
    unimplemented!();
}

/// `EnvSet`の読み込み～環境変数の設定～指定コマンドの実行、を行う。
fn run(env_set_name: &EnvSetName, cmd: &mut Command) -> Result {
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
        Err(EnvarsError::ProcessFail(exit_status))
    }
}

/// MODE別詳細ヘルプを出すか、一気にまとめて書くか
fn help() -> Result {
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
");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_anything() {}
}
