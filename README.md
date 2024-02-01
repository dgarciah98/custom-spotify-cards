# [Custom Spotify Card Generator](https://dgarciah98.github.io/custom-spotify-cards)

A Rust + WASM Yew app that generates a custom card image of your favorite song from Spotify.

You can find it here: https://dgarciah98.github.io/custom-spotify-cards

This is based on the same implementation I did for a Telegram bot, which can be found here https://github.com/dgarciah98/spotify_uri_bot/tree/image-generation


## How to use it
You're going to need a Spotify URI for the track you want. How do you get one?

- **Copy Song Link**: depending of your device, you can do the following
  - **Mobile**: push on the 'Share' button, then 'More' and finally 'Copy link'
  - **Desktop**: click the three dots next to the song, then 'Share' and 'Copy Song Link',
                 or copy directly from the browser
- **Copy Spotify URI**: on desktop, press ALT while going to the Copy Song Link button
- **Use Track ID**: taking only the ID itself for the track also works!

Keep in mind that it has to be a ***track***, that is the resulting URI has to be something like `open.spotify.com/track/<track_id>` or `spotify:track:<track_id>`

## TODO

- [x] Deploy to Github Pages
- [x] Add options to generate an image with different color backgrounds
- [ ] Add some examples to the readme
- [x] Definitely make the webpage less ugly
- [ ] Add more details to the readme
- [ ] Add explanations on how to run and set up the project
- [ ] Maybe upload a backend version???
