# Getting Started

## Development on localhost
Use `trunk` to compile to webassembly and run the webserver, with `--open` will tell to start the Browser with the URL listening on:
`trunk serve --open`
Use localhost to connect if `trunk` listens only on one ip stack.

## Deploy to Cloudflare Pages
Using `trunk` not only to build as preview but as well to deploy to cloudflare pages with `wrangler` is quite straight forward.
1. To deploy on Cloudflare build webassembly with the release mode:
   `trunk build --release`
2. To upload to cloudflare specify to output folder `dist`:
   `wrangler pages publish dist`
