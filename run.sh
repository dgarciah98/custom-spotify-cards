#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

# export CLIENT_ID=YOUR_CLIENT_ID
# export CLIENT_SECRET=YOUR_CLIENT_SECRET

(trap 'kill 0' SIGINT; \
 bash -c 'cd frontend; trunk serve --public-url "/custom-spotify-cards/"')
