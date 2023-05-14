# golden-god-bot
A Discord bot that quotes The Golden God from Always Sunny (inspired by u/golden-god-bot). Invite it to your server [here](https://discord.com/api/oauth2/authorize?client_id=1079555682723958794&permissions=274877925440&scope=bot%20applications.commands)!

This is just a project to help me learn Rust.

## Dependencies
- [serenity](https://github.com/serenity-rs/serenity)

## Environment Setup
Rename `.env.example` to just `.env` and open it up. Fill out the API Keys needed.

### Discord Applications
- Bot > Token > Copy > paste in a .env file
- OAuth2 > URL Generator > Scopes
  - Check `bot` and `applications.commands`
- OAuth2 > URL Generator > Bot Permissions
  - Check `Send Messages`, `Send Messages in Threads`, `Embed Links`, and `Add Reactions` 
- Copy the invite URL generated to add the bot to your own server

## Deployment
First thing's first: bots should be running 24/7 so you _should_ have access to a server. If you don't host a home server (well, you should) then a VPS may be a good option. From there, there are a lot of ways to deploy a Discord bot but I prefer [Docker](https://www.docker.com/).

The baseline Docker image is hosted on [Docker Hub](https://hub.docker.com/r/notchum/golden-god-bot). Use `notchum/golden-god-bot:latest` in your `docker run` command to pull that image.

### Building Custom Docker Image
Once in the `golden-god-bot` directory, build the image:
```sh
$ docker build -t golden-god-bot:<version> .
```

### Creating Docker Container
After you have the image, create & start a container:
```sh
$ docker run \
    --name golden-god-bot \
    --restart unless-stopped \
    -e TZ=<timezone> \
    -d golden-god-bot:<version>
```