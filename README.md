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

### Note: For Superteam

Hey Superteam, if you're reading this, I've successfully reached out to you. This bot is my contribution to the community, designed for those who don't want to miss any bounties or projects while they're away from their PC or your website. Personally, I missed many earning opportunities because I couldn't check the website frequently, and the Discord bot on the Earn Discord only notifies every 7 days, which wasn't enough for me. That's why I created this bot.