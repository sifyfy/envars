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

use command::Command;
use envset::EnvSet;
use std::env::Args;

pub enum CmdArgs {
    Edit(EnvSet),
    List,
    New(EnvSet),
    Run(EnvSet, Command),
    Help,
}

impl CmdArgs {
    pub fn from_args(args: &mut Args) -> CmdArgs {
        match args.next().and_then(|mode| Self::parse_args(&mode, args)) {
            Some(mode) => mode,
            None => CmdArgs::Help,
        }
    }

    fn parse_args(mode: &str, args: &mut Args) -> Option<CmdArgs> {
        match mode {
            "edit" => Self::parse_as_edit(args).map(CmdArgs::Edit),
            "list" => Some(CmdArgs::List),
            "new" => Self::parse_as_new(args).map(CmdArgs::New),
            "run" => Self::parse_as_run(args).map(|(env_set, cmd)| CmdArgs::Run(env_set, cmd)),
            _ => Some(CmdArgs::Help),
        }
    }

    fn parse_as_edit(args: &mut Args) -> Option<EnvSet> {
        args.next().map(|name| EnvSet::new(&name))
    }

    fn parse_as_new(args: &mut Args) -> Option<EnvSet> {
        args.next().map(|name| EnvSet::new(&name))
    }

    fn parse_as_run(args: &mut Args) -> Option<(EnvSet, Command)> {
        args.next().and_then(|name| {
            let mut cmd = String::new();
            for i in args {
                cmd.push_str()
            }
        })
        Some((EnvSet::new("aa"), Command::new("hoge")))
    }
}
