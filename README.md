## ## DEPENDENCIES ##
- `docker`
- `docker-buildx`

## ## HOW TO USE ##

1. `git clone https://github.com/TheBearodactyl/discord-autodeleter.git`
2. `cd discord-autodeleter`
3. `docker buildx build --tag "discordbot:0.1.0" --label "antispam bot"`
4. `docker run --detach --env RUN=true discordbot:0.1.0`
