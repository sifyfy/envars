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

extern crate yaml_rust;

pub mod cmdargs;
pub mod command;
pub mod envset;

use cmdargs::CmdArgs;
use command::Command;
use envset::EnvSet;

pub fn start(mode: CmdArgs) {
    match mode {
        CmdArgs::Run(env_set, command) => run(env_set, command),
        _ => println!("others"),
    }
}

fn run(env_set: EnvSet, command: Command) {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_anything() {}
}
