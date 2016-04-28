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

#![feature(custom_derive)]

extern crate s_app_dir;
extern crate yaml_rust;

pub mod cmdargs;
pub mod command;
pub mod envset;

use cmdargs::CmdArgs;
use command::Command;
use envset::EnvSet;
use s_app_dir::{AppDir, XdgDir};
use std::path::PathBuf;

pub fn start(mode: CmdArgs) {
    match mode {
        CmdArgs::Edit(env_set) => unimplemented!(),
        CmdArgs::List => unimplemented!(),
        CmdArgs::New(env_set) => unimplemented!(),
        CmdArgs::Run(env_set, command) => run(env_set, command),
        _ => unimplemented!(), // Show help.
    }
}

/// Get `$XDG_CONFIG_HOME`.
/// If Windows, get `%APPDATA%` instead of `$XDG_CONFIG_HOME`.
fn env_set_dir() -> PathBuf {
    AppDir::new("envars").xdg_dir(XdgDir::Config).expect("$XDG_CONFIG_HOME and $HOME are missing.")
}

fn run(env_set: EnvSet, command: Command) {
    let config_dir = env_set_dir();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_anything() {}
}
