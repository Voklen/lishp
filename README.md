# Lishp

A shell that neatly expands upon the basic form of a regular shell using a lisp-like syntax for more complex operations.

## Example

```
cd src/
ls
let default file1
if true (ls) (echo $default)
if false (ls) (echo $default)
```

## Docs
[Examples](docs/example_commands.md)

## Inspiration

The syntax of bash can be quite unergonomic and sometimes even cryptic, so it would be nice if we could have a regular programming language that we use as our shell. Something like [Xonsh](https://xon.sh/) is a good idea, but it feels like two seperate syntaxes in one language. So if we want a cohesive syntax, we can take a language like Python and use that syntax for everything but that would result in running commands looking like this:
```python
cd("src/")
ls()
command(arg1, arg2, arg3)
```
Which results in way too much verbosity for running shell commands.

The other option is to start with the bash syntax and then build a language off of that. So lets give that a go! In a regular shell like `sh` or `bash` you run a command like this:
```sh
command arg1 arg2 arg3
```
But you may notice, this is very similar to another family of languages: Lisps! If you imagine there are parentheses around the command:
```common-lisp
(command arg1 arg2 arg3)
```

In that case, if we just have implicit brackets around the top level expression, we can have bash syntax for moving around and running regular commands, but then use the power of a lisp syntax for more complex operations.
```
cd src/
ls
command arg1 arg2 arg3
command arg1 (subcommand arg2 arg3) arg4
```

And thus the idea for Lishp was born.

