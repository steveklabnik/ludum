# Ludum

My entry for Ludum Dare 35.

So, I ended up writing an interactive game engine, which I call “Ludlum”, and
then a game written in that engine, which I have titled “A Game.”

This is my first LD, and I’ve wanted to do it for a long time. However, since
it’s my first, and I didn’t even know that I was going to participate until
shortly before I got started, so I decided to pick something I’ve been doing
since I was a kid: text adventures, or “interactive fiction”, as they tend to
be called nowadays.

It’s a short game, but I decided to go meta: it’s a game loosely based on my
childhood, where you learn about programming games. “Transform”, the theme, is
more alluded to than it is direct: this is a game about how my own life was
transformed by computers.

Furthermore, I wrote the game in Rust, the programming language I work on as my
day job. It was a ton of fun. I picked a terminal library that apparently
doesn’t work with Windows, though, so that’s unfortunate. It should work on
Linux and Mac OS X.

You can build the game yourself. First, [install
Rust](https://www.rust-lang.org/downloads.html). Then, 
install with Cargo:

```bash
$ cargo install ludum
```

Then, make sure you have the `Game.toml` file:
https://github.com/steveklabnik/ludum/blob/master/Game.toml

Run `ludum` from that directory, and the game should start.

If you don’t have Rust installed, binary releases are on [the Releases
page](https://github.com/steveklabnik/ludum/releases).
