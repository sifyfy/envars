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

use envset::EnvSetName;
use std::env;
use std::process::Command;

pub enum CmdArgs {
    Edit(EnvSetName),
    List,
    New(EnvSetName),
    Run(EnvSetName, Command),
    Help,
}

impl CmdArgs {
    pub fn from_args() -> CmdArgs {
        let mut args = env::args().skip(1);
        match args.next().and_then(|mode| Self::parse_args(&mode, &mut args)) {
            Some(mode) => mode,
            None => CmdArgs::Help,
        }
    }

    fn parse_args<I>(mode: &str, args: &mut I) -> Option<CmdArgs>
        where I: Iterator<Item = String>
    {
        match mode {
            "edit" => Self::parse_as_edit(args).map(CmdArgs::Edit),
            "list" => Some(CmdArgs::List),
            "new" => Self::parse_as_new(args).map(CmdArgs::New),
            "run" => Self::parse_as_run(args).map(|(env_set, cmd)| CmdArgs::Run(env_set, cmd)),
            _ => Some(CmdArgs::Help),
        }
    }

    fn parse_as_edit<I>(args: &mut I) -> Option<EnvSetName>
        where I: Iterator<Item = String>
    {
        args.next().and_then(|name| EnvSetName::new(&name))
    }

    fn parse_as_new<I>(args: &mut I) -> Option<EnvSetName>
        where I: Iterator<Item = String>
    {
        args.next().and_then(|name| EnvSetName::new(&name))
    }

    fn parse_as_run<I>(args: &mut I) -> Option<(EnvSetName, Command)>
        where I: Iterator<Item = String>
    {
        args.next()
            .and_then(|name| EnvSetName::new(&name))
            .and_then(|env_set_name| {
                args.next().map(|program| {
                    let mut cmd = Command::new(&program);
                    for i in args {
                        cmd.arg(&i);
                    }
                    (env_set_name, cmd)
                })
            })
    }
}
