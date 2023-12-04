# My Advent of Code solutions (in Rust)

See [Advent of Code](https://adventofcode.com/).

### Puzzle input download script

Located [here](./bulk_download_puzzle_input), this script downloads all puzzle input and puts it in the [input](./input/) directory (mine is there in this repository). Time between downloads and which years to download can changed in the script. Puzzle input is user-specific, so a session id is needed. Get this from your browser while logged in.

Instructions (Firefox): On the [Advent of Code](https://adventofcode.com/) website, open the inspector (`F12`), go to the `Storage` tab, in `Cookies` click `https://adventofcode.com` and look for the value of the `session` cookie (a 128 characters long hex string at the time of writing). Run the downloader script and paste this session id when it asks for it.
