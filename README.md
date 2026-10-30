# downflac

a small music downloader written in rust that allows you to get almost any music in full FLAC quality for free.

## Build

```sh
cargo build --release
```

Binary at `target/release/downflac`.

## Usage

```sh
downflac [song name...]
```

or just 
```sh
downflac
```

- `-h`, `--help`: print help and exit.

- No args: prompts `Enter the name of your intended song:`.
- Args: all args joined by spaces as the query, e.g. `downflac never gonna give you up`.

Then:

1. Picks up to 5 Deezer matches.
2. Searches Kuwo for each.
3. Interactive select: `select a song` — list items `"<title> by <artist> (<album>)"`.
4. Downloads the chosen result to `"<title> by <artist> (<album>).<format>"` in the current dir.
5. embeds the song's cover art into the file

## Notes

- Output filename is the raw selection label plus format, e.g. `Song by Artist (Album).mp3`.
- On success prints `Download Completed`; errors are printed but the process still exits 0.
