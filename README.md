# hit-this-shi

A tiny Rust server that gives you **one personalized message** — and you’re stuck with it.  
Refreshing won’t help. Different device? Different fate.

This is a fun, intentionally minimal project built for vibes, not scale.

---

## What is this?

`hit-this-shi` is a simple HTTP server that returns **exactly one message per user**, chosen deterministically based on:

- your **IP address**
- your **User-Agent**

The same inputs will always lead to the same message (unless the server restarts), which makes the result feel oddly personal.

No accounts.  
No tracking.  
No databases.  
Just fate and a hash function.

---

## How it works (high level)

1. The server collects:
   - Client IP address
   - User-Agent header
2. These are combined into a fingerprint string
3. The fingerprint is hashed using Rust’s `DefaultHasher`
4. The hash is mapped to an index in a predefined list of messages
5. That message is returned

To keep things interesting:
- Once a message is assigned, it is **temporarily retired**
- The same message will **not be reused for the next 10 requests**
- After that, it can appear again

This avoids boring repetition while keeping the system stateless and simple.

---

## Privacy & Ethics

Yes — this project uses your **IP address**.

Important clarifications:

- The IP is **not stored**
- The User-Agent is **not stored**
- No logs are kept
- No identifiers are persisted
- Everything happens **in memory**, per request

The IP is only used momentarily to compute a hash that selects a message.

This project is meant to be fun, transparent, and respectful — not creepy.

---

## Tech Stack

- **Rust**
- **Axum** — HTTP server framework
- **Tokio** — async runtime
- **once_cell** — for minimal global state
- **In-memory logic only** (no DB, no Redis)

---

## Running locally

### Prerequisites
- Rust (stable)
- Cargo

### Run
```bash
cargo run
```
The server will start on port 3000. Open it in your browser and accept your fate.

---

## Disclaimer

This project is:
- Not meant for production use
- Not meant for analytics
- Not meant for scaling
- Absolutely meant to make you feel called out

