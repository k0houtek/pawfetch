# pawfetch
complete rust noob's attempt at making a fetch script as practice

the code is terrible but it works :p

## configuration!
pawfetch can be configured via either these config files:
- `~/.config/pawfetch/ascii_art` for the ASCII art
- `~/.config/pawfetch/infolines` for the infolines, which control the info to display

or these environment variables
- `PAWFETCH_ASCIIART`
- `PAWFETCH_INFOLINES`

the format of the infolines config is a comma-separated list of `title:content` pairs, where content can be:
- `os`
- `cpu`
- `memory`
- `swap`
- `user`
- `hostname`
- `userhost`
- blank for an empty line

and title can be any string (leave blank to make the content line show up on its own, in bold)
