# fromis

[![CI](https://github.com/m1sk9/fromis/actions/workflows/ci.yaml/badge.svg)](https://github.com/m1sk9/fromis/actions/workflows/ci.yaml)
[![Release fromis](https://github.com/m1sk9/fromis/actions/workflows/release.yaml/badge.svg)](https://github.com/m1sk9/fromis/actions/workflows/release.yaml)

Discord Bot to extract source code from permalink on GitHub.

> [!NOTE]
> 
> This project is still in development and may not be stable.

```shell
# latest version
docker pull ghcr.io/m1sk9/fromis:latest

# Minor version (v0.x) - recommended
docker pull ghcr.io/m1sk9/fromis:v0
```

*API Support: requires Discord API v10*

## Setup

1. [Create a Discord Application and Bot](https://discord.com/developers/applications).
   - Enable `MESSAGE CONTENT INTENT`.
2. Invite the bot to your server.
   - fromis requires `View Channels`, `Send Messages` permissions.
3. Create a `.env` and `compose.yaml` files.
   
    ```shell
    DISCORD_API_TOKEN=
    ```
   
    ```yaml
    services:
      app:
        image: ghcr.io/m1sk9/fromis:v0
        env_file:
          - .env
        restart: always
    ```

4. Run the bot.

    ```shell
    docker-compose up -d
    ```
   
## Usage

fromis sends the code part to Discord when you send a permalink. You can see the code without clicking the link.

If you enclose the link in `<>`, fromis will ignore it without expanding it.


```markdown
https://github.com/<owner>/<repo>/blob/<git-hash>/<file-name>#<line-number>
```

## License

fromis is published under [MIT License](LICENSE).

<sub>
    © 2024 m1sk9 - <a href="https://www.youtube.com/watch?v=0LiQp7y8Wwc" target="_blank">Origin of fromis?</a>
    <br>
    fromis is not affiliated with Discord or GitHub.
</sub>


