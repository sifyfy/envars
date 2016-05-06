# envars

envars run any command with predefined environment variables.

Predefined environment variables is saved on a yaml file (is called EnvSet) before running with any command.

## Install

`cargo install --git https://github.com/siphilia/envars`

## Usage

* `envars run ENV_SET_NAME COMMAND`
* `envars list`
* `envars new ENV_SET_NAME`
* `envars edit ENV_SET_NAME`
* `envars help`

`edit` mode open the EnvSet file with the editor (defined `$EDITOR` or `%EDITOR%`).

## Examples

1. Init new `foo` EnvSet: `envars new foo`
2. Edit `foo` EnvSet: `envars edit foo`
3. Run any command with `foo` EnvSet: `envars run foo command`

## LICENSE

```
The MIT License (MIT)

Copyright (c) 2016 Siphilia

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
