# pawfetch
complete rust noob's attempt at making a fetch script as practice

the code is terrible but it works :p

## configuration!
currently pawfetch can be configured via 2 environment variables
- `PAWFETCH_ASCIIART` to set the ASCII art to use
- `PAWFETCH_INFOLINES` to control what information to display
the format of `PAWFETCH_INFOLINES` is a comma-separated list of `title:content` pairs, where content can be:
    - `os`
    - `cpu`
    - `memory`
    - `swap`
    - `user`
    - `hostname`
    - `userhost`
    - blank for an empty line

and title can be any string (leave blank to make the content line show up on its own, in bold)
