# benson
Benson, the light that warms all


## Commands

| Command                | Info                             |
|------------------------|----------------------------------|
| `!benson_ping`         | Checks benson bot status         |
| `!benson_join_vc <id>` | Joins a voice channel by ID      |
| `!benson_leave_vc`     | Leaves the current voice channel |

## Running in Debug mode

```sh
DISCORD_TOKEN=<token> DISCORD_APP_ID=<app_id> RUST_LOG=info cargo run -- ./config.json
```
