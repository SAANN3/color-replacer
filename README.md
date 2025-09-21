# Color replacer
A program that extracts colors from an image, modifies them, and inserts the selected colors into other files.

![preview](./readme/preview.gif)
## How to build
```bash
git clone https://github.com/SAANN3/color-replacer
cd color-replacer
cargo build --release
```
The built binary will be located in ```target/release/color-replacer```

## Usage
### Tui
1. Enter the absolute path to the image you want to use, then press Continue.
2. On the second page, choose which colors to assign to each key (primary, secondary, tertiary).
3. Adjust colors by pressing +(or =) to lighten the selected color and - to darken.
4. When you are ready, press replace button.
### Cli 
Pass --cli flag for cli mode and --image, which will be used. unlike the tui, you can't select or modify colors (Will be fixed in future! (ᵕ—ᴗ—))
```bash 
color-replacer -c -i /path/to/image
```


## Configuration
At first start it will create a config file (or you can create it manually) in ```~/.config/colors_replacer/config.json```. Then you put in it files that will be modified
```json
{
  "warning": {
    "first_time": false, // <-- set it to false to continue
    "text": "Set first_time to false in order to continue!"
  },
  "files": [ // array of files that will be processed
    {
      "from": "/absolute/path/from", // file that contains keys, which will be replaced 
      "to": "/absolute/path/to" // file that will be created|modified as output
    }
  ]
}
```
for example, 'from' file
```
my_first_color = $[primary]
my_second_color = $[secondary]
third = $[tertiary]
$[primary]|$[secondary]|$[tertiary]
```
and output, file 'to'
```
my_first_color = #FFFFEA
my_second_color = #D79475
third = #B6AADE
#FFFFEA|#D79475|#B6AADE
```
## Params
```
-p, --path-cfg <PATH_CFG>  Custom path to config file
-c, --cli                  Enables cli mode
-i, --image <IMAGE>        Path to image that will be used in cli mode or opened in tui
-s, --silence              Silence all output in cli mode
-h, --help                 Print help
-V, --version              Print version
```
Note that cli mode requires image param to work

## License

Copyright (c) SAANN3 <95036865+SAANN3@users.noreply.github.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
