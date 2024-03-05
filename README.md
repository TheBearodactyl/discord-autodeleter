## -- *DEPENDENCIES* -- ##
- `docker`
- `docker-buildx`

## -- *HOW TO USE* -- ##
```bash
git clone https://github.com/TheBearodactyl/discord-autodeleter.git
cd discord-autodeleter
echo "DISCORD_TOKEN=\"YOUR_BOT_TOKEN_HERE\"" >> .env
docker buildx build --tag "discordbot:0.1.0" --label "antispam bot"
docker run --detach --env RUN=true discordbot:0.1.0
```
