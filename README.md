# Pip Macros for Elite: Dangerous (Linux/X11)

> ℹ️  if you are using Wayland, this will probably still work assuming you have xwayland installed.

## About 
This is a tool which will listen for Presses on certain keys (1-3 by default), and will then spam other keys (left, top and right arrow by default) as long as those keys are pressed.
The "spamming" will only take place if
- the active window is Elite: Dangerous (the window name is `steam_app_359320`)
- the Tool is active (you can toggle this by pressing ` - ` on the numpad)

## Requirements 
Assumes X11 / Linux. No support for Mac / Windows / whatever.

If you want to build from source, you might need to install these packages:  
`libasound2-dev build-essential libxi-dev libxtst6 pkg-config xorg-dev` (apt) or the equivalent for your Package Manager.

## Installation

### Binary
You can download the binary straight from Github and place it somewhere in `$PATH`.

### From Source
`cargo install --git https://github.com/CMDR-WDX/elite-pip-macro`.
Make sure `~/.cargo/bin` is in your `$PATH` (or call it directly: `~/.cargo/bin/elite-pip-macro`

See *Requirements* Section above in case the build fails because of system dependencies.

# From AUR

## Binary Install 

Alternatively you can install the binary [AUR](https://aur.archlinux.org/packages/elite-pip-macro-bin) package.

### Paru
```
paru -S elite-pip-macro-bin
```
### Yay
```
yay -S elite-pip-macro-bin
``` 

## From source

You can also install from the [AUR](https://aur.archlinux.org/packages/elite-pip-macro).

### Paru
```
paru -S elite-pip-macro
```
### Yay
```
yay -S elite-pip-macro
``` 

> [!NOTE]  
> This will compile the package from source. If you do not want to compile the program, then please use the binary provided under  [Releases](https://github.com/CMDR-WDX/elite-pip-macro/releases) or the binary [AUR](https://aur.archlinux.org/packages/elite-pip-macro-bin) package.

## Usage
by default the Macro is set up to 
* Spam `LeftArrow` if `1` is pressed (SYS)
* Spam `UpArrow` if `2` is pressed (ENG)
* Spam `RightArrow` if `3` is pressed (WEP)

To effectively use this setup you want to **unbind** the UI Focus to Left, Chat, Bottom and Right Panel. 
I recommend rebinding in-game actions from `1`, `2`, `3` and `4` to `F1`, `F2`, `F3` and `F4` respectively.

You can also define your own inputs and outputs. See `elite-pip-macro run --help` for more info.
To override the keys, you need to get the keycode. Use `elite-pip-macro key-repl`. This will print the Key and Key Code of any key you press.  
**Keep in mind that the Name does not neccesarily represent the Key "value".** For example, if I press `Y` on my German keyboard, `key-repl` will return `Pressed key KeyZ, which has Keycode "52"`.
Keycodes define the Key Position, not the Key Value. So simply press the key you want to use and note its keycode.
You can then use the `run` command and pass the modified configuration. If you (for whatever reason) want to Press `Q` to put pips to ENG, you can `elite-pip-macro run --in-eng=24`.
Generally, `--in-*` is the key you have to press, while `--out-*` is the key that gets spammed.
