# weirdgrep

Weirdgrep is a tool that allows you to search through huge files (initially designed to look through pages of code).

# Usage

```bash
❯ weirdgrep -h
weirdgrep 1.0.0
Vaelio <archelio@protonmail.com>
Regex parser to search through files.

USAGE:
    weirdgrep [OPTIONS] <REGEX> <ENDTAG> <PATH>

ARGS:
    <REGEX>     Regex to apply
    <ENDTAG>    End Tag for the match
    <PATH>      File to parse

OPTIONS:
    -h, --help         Print help information
    -n, --numbers      Print line numbers for each printed lines starting from 0
    -V, --version      Print version information
    -w <WITHIN>        Switch to scope mode, and use this regex as a search and (regex, endtag) as
                       boundaries of the search afterwards
```

By default the tool will search for \<REGEX\> inside \<PATH\> and will print each matches until \<ENDTAG\> is reached.
(This \<ENDTAG\> can be a regex aswell)

It is usefull when you want to extract for exemple every functions of a code page where the signature matches \<REGEX\>

You can further improve the granularity by adding the -w \<WITHIN\> switch which makes the tool search for this regex instead
and then go forward and backward to respectively find \<ENDTAG\> and \<REGEX\> args. 
(This \<WITHIN\> option can also be a regex)
