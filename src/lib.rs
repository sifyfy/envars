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
pub mod command;
pub mod envset;

use cmdargs::CmdArgs;
use command::Command;
use envset::{EnvSet, EnvSetError, EnvSetName};
use std::error::Error;
use std::fmt;
use std::io;
use std::result;

#[derive(Debug)]
pub enum EnvarsError {
    EnvSet(EnvSetError),
    IO(io::Error),
}

impl fmt::Display for EnvarsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnvarsError::EnvSet(ref e) => write!(f, "EnvarsError(EnvSetError): {}", e),
            EnvarsError::IO(ref e) => write!(f, "EnvarsError(IO): {}", e),
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
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            EnvarsError::EnvSet(ref e) => Some(e),
            EnvarsError::IO(ref e) => Some(e),
        }
    }
}

pub type Result<T> = result::Result<T, EnvarsError>;

pub fn start(mode: CmdArgs) -> Result<()> {
    match mode {
        CmdArgs::Edit(env_set) => unimplemented!(),
        CmdArgs::List => unimplemented!(),
        CmdArgs::New(env_set) => unimplemented!(),
        CmdArgs::Run(env_set, command) => run(env_set, command),
        _ => unimplemented!(), // Show help.
    }
}

/// `EnvSet`の読み込み～環境変数の設定～指定コマンドの実行、を行う。
fn run(env_set_name: EnvSetName, command: Command) -> Result<()> {
    let env_set: EnvSet = try!(EnvSet::new(&env_set_name));
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_anything() {}
}
