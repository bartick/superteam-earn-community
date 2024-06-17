# EARN BOT

This is an unofficial earn bot that monitors the Superteam Earn website and notifies you when a new bounty or project becomes available.

## Install and Run

Clone the repository

```bash
git clone https://github.com/bartick/superteam-earn-community.git
```

Set environment variables

```bash
# set environment variables
cp .env.example .env
```

Run the bot in development or production mode
```bash
# development mode
cargo build
cargo run

# production mode
cargo build --release
cargo run --release
```

## Extra (Installation)

If you do not have the database setup yet, you need to install PostgreSQL and create a database.

### 1. Install PostgreSQL

You need to install postgresql on your machine. You can follow the instructions on the [official website](https://www.postgresql.org/download/).
And install postgresql client psql (linux) or libpq (macos) or pgAdmin (windows).

### 2. Migration and Database Setup 
```bash
# install diesel_cli
cargo install diesel_cli --no-default-features --features postgres

# create database
diesel migration run
```

## Invite the bot to your server

1. Discord: [Bounty Bot](https://discord.com/oauth2/authorize?client_id=1249051949471629434&permissions=137439463488&integration_type=0&scope=bot+applications.commands)

2. Telegram: [Bounty Bot](https://t.me/superbountybot)



#### Note: For Superteam

Hey Superteam, if you're reading this, I've successfully reached out to you. This bot is my contribution to the community, designed for those who don't want to miss any bounties or projects while they're away from their PC or your website. Personally, I missed many earning opportunities because I couldn't check the website frequently, and the Discord bot on the Earn Discord only notifies every 7 days, which wasn't enough for me. That's why I created this bot.