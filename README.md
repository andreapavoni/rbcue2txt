# RbCue2Txt

Parses cue files from Rekordbox's recordings to produce readable, markdown-friendly playlists.

## Usage

```sh
rbcue2txt <path to cue file>
```

Will output the playlist in this format:

```
- 00:00:02: Giuseppe Surace - Shot You Down (Original Mix)

- 00:03:06: Aldo Cadiz - Tzu-Mani

- 00:07:49: Gerd Janson - Never Saw Never

- 00:10:49: Alberto Dimeo - Fuego Fuego

- 00:14:23: Aitor Astiz - Ta Gringa

- 00:17:18: Donato Dozzy - Parola (Black Fancy Mix)

- 00:21:16: Superlover - Piano Pump

- 00:27:57: Madison Avenue - Don't Call Me Baby (Joshwa Remix)

- 00:30:02: Django Django - Don't Touch That Dial (feat. Yuuko Sings) (Earthworks Acid Hip-House Remix)

- 00:32:02: Anatta - Tobacco Docks

- 00:33:24: Django Django - Don't Touch That Dial (feat. Yuuko Sings) (Earthworks Acid Hip-House Remix)

- 00:36:02: COLOR.LOVE - Bad Bitch (Original Mix)

- 00:39:17: Uffie - ADD SUV (feat. Pharrell Williams)

- 00:42:53: Spellband,ManuLele - DiscoParty - Monkey Mix

- 00:47:02: DANK.K - Simple Rave In a Buffalo Herd

- 00:52:36: Mathias Kaden - Red Walls

- 00:57:02: Super Flu - Super Flu 3 < Isaac

```

And in markdown it would be like this:

- 00:00:02: Giuseppe Surace - Shot You Down (Original Mix)

- 00:03:06: Aldo Cadiz - Tzu-Mani

- 00:07:49: Gerd Janson - Never Saw Never

- 00:10:49: Alberto Dimeo - Fuego Fuego

- 00:14:23: Aitor Astiz - Ta Gringa

- 00:17:18: Donato Dozzy - Parola (Black Fancy Mix)

- 00:21:16: Superlover - Piano Pump

- 00:27:57: Madison Avenue - Don't Call Me Baby (Joshwa Remix)

- 00:30:02: Django Django - Don't Touch That Dial (feat. Yuuko Sings) (Earthworks Acid Hip-House Remix)

- 00:32:02: Anatta - Tobacco Docks

- 00:33:24: Django Django - Don't Touch That Dial (feat. Yuuko Sings) (Earthworks Acid Hip-House Remix)

- 00:36:02: COLOR.LOVE - Bad Bitch (Original Mix)

- 00:39:17: Uffie - ADD SUV (feat. Pharrell Williams)

- 00:42:53: Spellband,ManuLele - DiscoParty - Monkey Mix

- 00:47:02: DANK.K - Simple Rave In a Buffalo Herd

- 00:52:36: Mathias Kaden - Red Walls

- 00:57:02: Super Flu - Super Flu 3 < Isaac

---

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
