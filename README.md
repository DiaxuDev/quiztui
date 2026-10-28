# quiztui

Flashcards in your terminal

## How does it work?

Quiztui is a really simple app that makes flashcards out of a csv file. For every flashcard you're shown the word's explanation (usually backside of a typical flashcard) and you have to type the word itself (usually frontside of a typical flashcard). If you get it right the flashcard is removed for the duration of the entire study session. If you get it wrong though you will be shown the word and will have to retype it into the textbox and the flashcard will be shown again another time. Aditionally quiztui tries to automatically do some basic parsing so for example if the expected word is `foo/a` you can type either `foo` or `foa`.

## Installation

For Nix users this repo provides a flake which you can include in your configuration or test it out with `nix shell`.

```sh
$ nix shell github:DiaxuDev/quiztui
```

If you want to compile it yourself, see [Building](#building).

## Usage

To begin you must have a file named `input.csv` in your current working directory containing your flashcards separated by a newline.
Example:

```csv
frontside1,backside1
frontside2,backside2
```

With the file present all you have to do now is run the binary and you will be shown the main UI.

```sh
$ quiztui
```

## Planned features

- configuration
- integrated flashcard management with an UI
- study session retainment between command executions

## Building

For Nix users there is a provided devshell which you can use, either with nix-direnv by running `direnv allow` or manually with `nix develop`.

If you don't want to use the devshell you will need rust and cargo installed on your system. To build `quiztui` locally one must simply run

```sh
$ cargo build
```
