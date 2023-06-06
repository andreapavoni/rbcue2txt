# RbCue2Txt

Parses cue files from Rekordbox's recordings to produce readable, markdown-friendly playlists.

I needed a similar utility, I've seen there are many of them out there, but I wanted to make a more portable one while playing with Rust.

## Usage

```sh
rbcue2txt <path to cue file>
```

By default, it will output the playlist in this format:

```
- [00:00:02] Giuseppe Surace - Shot You Down (Original Mix)
- [00:03:06] Aldo Cadiz - Tzu-Mani
...
- [00:57:02] Super Flu - Super Flu 3 < Isaac
```

And in markdown it would look like this:

- [00:00:02] Giuseppe Surace - Shot You Down (Original Mix)
- [00:03:06] Aldo Cadiz - Tzu-Mani
...
- [00:57:02] Super Flu - Super Flu 3 < Isaac


However, you can also pass a desired output format as second argument, by following these rules:

- `%A`: artist name
- `%T`: time
- `%N`: title

So, for example, calling `rbcue2txt <path to cue file> "%A - %N"` will produce this output:

```
Giuseppe Surace - Shot You Down (Original Mix)
Aldo Cadiz - Tzu-Mani
...
Super Flu - Super Flu 3 < Isaac
```

### Notes

Tested against Rekordbox 6 cue sheets on MacOS.

## Installation

### Precompiled binaries

Download the binary for your platform from [https://github.com/andreapavoni/rbcue2txt/releases](https://github.com/andreapavoni/rbcue2txt/releases).

### From sources

- clone this repository: `git clone https://github.com/andreapavoni/rbcue2txt.git`
- cd into the directory: `cd rbcue2txt`
- test: `cargo test`
- build: `cargo build --release`
- Move the binary to `/usr/local/bin`: `mv target/release/rbcue2txt /usr/local/bin/rbcue2txt`


## Credits

The authors of [rekordbox-cue-to-txt](https://github.com/keyle/rekordbox-cue-to-txt/tree/master) for their original work in JS.

---

©2023 a [pavonz](https://pavonz.com) joint
