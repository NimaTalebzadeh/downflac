# downflac

CLI music downloader. Search Deezer for a song, resolve a matching Kuwo stream via `flac.music.hi.cn`, download to disk with a progress bar.

## Install

```sh
cargo build --release
```

Binary at `target/release/downflac`.

## Usage

```sh
downflac [song name...]
```

- `-h`, `--help`: print help and exit.

- No args: prompts `Enter the name of your intended song:`.
- Args: all args joined by spaces as the query, e.g. `downflac never gonna give you up`.

Then:

1. Picks up to 5 Deezer matches.
2. Searches Kuwo on `flac.music.hi.cn` for each.
3. Interactive select: `select a song` — list items `"<title> by <artist> (<album>)"`.
4. Downloads the chosen result to `"<title> by <artist> (<album>).<format>"` in the current dir.

## Notes

- Requires network access to `api.deezer.com` and `flac.music.hi.cn`.
- Output filename is the raw selection label plus format, e.g. `Song by Artist (Album).mp3`. Not sanitized — titles containing `/` or similar can fail.
- On success prints `Download Completed`; errors are printed but the process still exits 0.
