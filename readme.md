<p align="center">
    <h1 align="center">dotman</h1>
</p1>

<p align="center"><i>dotman is dotfiles manager.</i></p>

<p align="center">
    <a href=".license-mit"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a> 
</p>

## Installation via Cargo
```
❯ git clone https://github.com/atsushi130/dotman
❯ cargo install
```

## Usage
#### Commands
- install
  + vimrc
  + zshrc
  + gitconfig
  + gitignore
  + ... 

```
❯ dotman install vimrc
```

## Settings
`settings.json`
```
{
  "repository": "account/repository",
  "dotfiles": [
    {
      "name": "vimrc", // <- install argument name
      "input": "/vim/.vimrc", // <- ex: https://github.com/account/repository/blob/master/vim/.vimrc
      "output": "~/.vimrc" // <- install path
    }
  ]
}
```

## License
dotman is available under the MIT license. See the [LICENSE file](https://github.com/atsushi130/dotman/blob/master/license-mit).
