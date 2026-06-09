# My scrobbler (for Apple Music)

### Install

```sh
brew tap mishazawa/mishazawa
brew install mishazawa/mishazawa/my-scrobbler
```

### Config

[Create Last.fm API keys](https://www.last.fm/api#getting-started)

```sh
# add password to keychain
security add-generic-password -U -s "my-scrobbler" -a <USERNAME> -w <PASSWORD>
```

```sh
# ~/.config/my_scrobbler/config.env
LASTFM_API_KEY="your_api_key"
LASTFM_API_SECRET="your_secret"
LASTFM_USERNAME="your_username"
```
