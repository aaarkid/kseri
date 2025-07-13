# Kseri WebSocket Game Server

This WebSocket server manages multiplayer Kseri card game sessions with authoritative game state.

## Features

- **Two-player matchmaking**: First player waits, second player triggers game start
- **Authoritative game state**: All moves validated server-side
- **Graceful reconnection**: 30-second timeout for disconnected players
- **Real-time state sync**: Efficient broadcasting to both players
- **Full Kseri rules**: Including special scoring and Kseri bonuses

## Architecture

The server is implemented as a binary within the main project to reuse card components:

```
src/
├── server/
│   ├── mod.rs              # Module exports
│   ├── connection.rs       # WebSocket connection handling
│   ├── session.rs          # Game session management
│   ├── messages.rs         # Protocol definitions
│   ├── game_engine.rs      # Kseri game logic
│   └── broadcaster.rs      # State synchronization
└── bin/
    └── server.rs           # Server entry point
```

## Running the Server

```bash
# Run with default settings (port 8080)
cargo run --bin kseri-server

# Run with custom port
KSERI_PORT=9000 cargo run --bin kseri-server

# Run with debug logging
RUST_LOG=debug cargo run --bin kseri-server
```

## Environment Variables

- `KSERI_PORT`: Server port (default: 8080)
- `KSERI_LOG_LEVEL`: Log level (default: info)
- `KSERI_MAX_CONNECTIONS`: Max concurrent connections (default: 1000)
- `KSERI_SESSION_TIMEOUT_SECS`: Session timeout in seconds (default: 1800)

## WebSocket Protocol

### Client Messages

```typescript
type ClientMessage = 
  | { type: "JoinQueue", data: { player_name: string } }
  | { type: "Reconnect", data: { session_token: string } }
  | { type: "PlayCard", data: { card: Card } }
  | { type: "RequestState", data: {} }
  | { type: "Ping", data: {} }
```

### Server Messages

```typescript
type ServerMessage =
  | { type: "Connected", data: { player_id: number, session_token: string } }
  | { type: "WaitingForOpponent", data: {} }
  | { type: "GameStarted", data: { opponent_name: string, your_turn: boolean, initial_hand: Card[], table_cards: Card[] } }
  | { type: "CardPlayed", data: { player: number, card: Card, captured_cards: Card[], is_kseri: boolean } }
  | { type: "StateUpdate", data: { /* game state fields */ } }
  | { type: "GameOver", data: { winner: number | null, final_scores: [number, number] } }
  | { type: "Error", data: { message: string } }
```

## Testing

```bash
# Run unit tests
cargo test game_engine

# Run integration test
cargo test server_integration

# Run example client
cargo run --example test_client
```

## Production Considerations

1. **TLS/WSS**: Use a reverse proxy (nginx/caddy) for SSL termination
2. **Scaling**: Currently single-server; use Redis for multi-server sessions
3. **Monitoring**: Integrate with Prometheus/Grafana for metrics
4. **Rate Limiting**: Add per-IP rate limiting for DDoS protection
5. **Authentication**: Add JWT tokens for player authentication