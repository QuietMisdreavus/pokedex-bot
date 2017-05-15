# pokedex-bot

An IRC bot that dispenses Pokemon information

This is a rather rough first attempt at an IRC bot, using information from [eevee's pokedex
dump][pokedex] as the data source. It loads the CSV files directly, so you don't need to use their
Python setup program before running the bot. You will need to set up the `config.json` file before
running, though, but there's an example file in the repo with the basics ready to edit. The steps
required to set up should be as follows:

[pokedex]: https://github.com/veekun/pokedex

```sh
git clone --recursive https://github.com/QuietMisdreavus/pokedex-bot.git
cd pokedex-bot
cp config.example.json config.json
$VISUAL config.json # set up handle/server/channel/etc
cargo run
```

Right now this is incredibly basic - the only thing the bot will display is the Pokemon's name,
classification, and type(s). I hope to add more features in the near future. The basic interaction
involves issuing a search with `!dex [name]` or `!dex [number]`. Some example output follows:

```
<misdreavus> !dex arceus
<[myBot]> #493 Arceus, The Alpha Pokemon, Normal type
<misdreavus> !dex 123
<[myBot]> #123 Scyther, The Mantis Pokemon, Bug/Flying type
```
