# Maty's homepage

Yo, heres the source of my website if you are curious

## But maty why did you use rust? Don't you know its a systems programming language and it's terrible for UIs?

Do you know whats terrible for my migraine? Javascript. I prefer to keep my sanity and bear through front end rust development than rip my eyes out over ANY amount of javascript code. Don't you get me started on typescript... Javascript is so SHIT they made a language on top of it just to not use it. I understand the impulse, but its kind of hilarious. Plus I will take rusts single management tool cargo over one and a half million tools needed to have a bearable js/ts developer experience thank you very much.

## Development

1. Install rust
2. Install dioxus dx tool - for example via `cargo binstall`.
3. Install just
4. `just serve` for development on localhost
5. `just bundle_web` for release zip

### If you are using nix

There is a flake.nix for development. I did rely on installing dx via cargo tho.

## Adding more 'captcha'd contacts

There is no automatic way, I just used a test case with a `dbg!` line to get the encoded bytes.