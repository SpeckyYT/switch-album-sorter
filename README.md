# Switch Album Sorter

This program sorts your Nintendo Switch Album folder.

By default, if you put the executable file in the same directory where your
`Album` folder is located, then it will create a `Sorted` folder and put
the screenshots and clips there.

## How can I customize the input and output folders?

If you run this program via command line, you can add the flags
`-i "/path/to/album/folder" -o "/path/to/output/folder"` to
customize input and output.

## How to add custom game names?

By adding a `game_ids.json` file, you can customize the game's names.

The following two JSON structures are allowed:

```json
{
    "GAME ID": "GAME NAME",
    "GAME ID": "GAME NAME",
}
```

```json
[
    ["GAME ID", "GAME NAME"],
    ["GAME ID", "GAME NAME"],
]
```

[Here's a list you can use](https://github.com/s1cp/nxshot/blob/master/gameids.json)
