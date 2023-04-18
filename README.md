# Custom Spotify Card Generator

A Rust + WASM Yew app that generates a custom card image of your favorite song from Spotify.

This is based on the same implementation I did for a Telegram bot, which can be found here [https://github.com/dgarciah98/spotify_uri_bot/tree/image-generation](https://github.com/dgarciah98/spotify_uri_bot/tree/image-generation)

You can find it here: https://dgarciah98.github.io/custom-spotify-cards

## How to use it
You're going to need a Spotify URI for the track you want. How do you get one?

- **Copy Song Link**: just copy the link from your app like you normally would or from your browser
- **Copy Spotify URI**: on desktop, press ALT while going to the Copy Song Link button
- **Use Track ID**: taking only the ID itself for the track also works!

Keep in mind that it has to be a ***track***, that is the resulting URI has to be something like `open.spotify.com/track/<track_id>` or `spotify:track:<track_id>`

## TODO

- [x] Deploy to Github Pages
- [ ] Add options to generate an image with plain color background (or inverted gradient)
- [ ] Add some examples to the readme
- [ ] Definitely make the webpage less ugly
- [ ] Add more details to the readme
- [ ] Add explanations on how to run and set up the project
- [ ] Maybe upload a backend version???
