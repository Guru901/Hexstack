# HexStack

[![Crates.io](https://img.shields.io/crates/v/hexstack)](https://crates.io/crates/hexstack)

### A modern, full-stack framework and CLI for Rust

Please star the repo if you like it, so that I know someone is using it.

> **Project Status**
>
> HexStack is **not abandoned**. Development is currently quiet because the project has reached the point where I don't have anything meaningful that needs to be added right now.
>
> HexStack was built to provide a simple way to build full-stack Rust applications using Ripress and Wynd, with Lume as an optional data layer. The core functionality is there and the CLI does what it was built to do.
>
> I'm not interested in adding features just to keep the commit history active. If there's something that genuinely needs to be improved, I'll work on it.
>
> The project is still maintained, and I'm happy to make changes when they're actually needed.

## What is HexStack?

HexStack is two things at once: a **full-stack Rust framework** that combines the familiar developer experience of Express.js, WS, and Drizzle with the performance and safety of Rust, and a **CLI tool** (`hexstack new`) that scaffolds a working project on top of it in seconds.

HexStack brings together three libraries to create a unified full-stack development experience:

* **[Ripress](https://github.com/guru901/ripress)** - HTTP server with an Express.js-inspired API
* **[Wynd](https://github.com/guru901/wynd)** - Simple, powerful WebSocket server
* **[Lume](https://github.com/guru901/lume)** - Type-safe ORM with an intuitive API *(experimental)*

## Why Choose HexStack?

**Performance**

* HTTP performance: 97% of Actix-Web speed
* WebSocket performance: Built on Tokio for maximum async efficiency
* Nearly 10x faster than Express.js + Socket.io

**Developer Experience**

* Express.js-familiar API that feels natural
* Type-safe from the ground up
* Comprehensive CLI for rapid scaffolding
* Seamless integration between all components

**Modern Architecture**

* Async/await support throughout
* HTTP/2 support via Hyper
* Real-time WebSocket capabilities
* Production-ready HTTP and WebSocket layers from day one

![Performance Benchmark](./benchmark.png)

## Table of Contents

* [Overview](#overview)
* [Features](#features)
* [Installation](#installation)
* [CLI Usage](#cli-usage)
* [Examples](#examples)
* [Documentation](#documentation)
* [Ecosystem](#ecosystem)
* [Changelog](#changelog)

---

## Overview

HexStack is designed to be the Rails/Laravel of the Rust ecosystem — a complete, opinionated framework paired with a CLI that scaffolds new projects in seconds so you can start building immediately.

The HTTP and WebSocket layers (Ripress + Wynd) provide the core functionality HexStack was built around, while the ORM layer (Lume) remains experimental — see [Ecosystem](#ecosystem).

HexStack is currently **feature-complete for its existing scope**. There isn't an active stream of new development because there isn't a need for one.

That doesn't mean the project is abandoned.

If something needs to be fixed, an existing component needs to adapt, or there's a meaningful improvement worth making, I'm happy to maintain the project and work on it.

---

## Features

* **Powerful CLI** - Scaffold projects with `hexstack new`
* **HTTP + WebSocket** - Unified server handling both protocols
* **Zero Configuration** - Works out of the box with sensible defaults
* **Multiple Templates** - HTTP-only, WebSocket-only, or full-stack apps
* **Type Safety** - Compile-time guarantees throughout the stack
* **Built-in Middleware** - CORS, logging, file uploads, rate limiting
* **Real-time Ready** - WebSocket support for live features

---

## Installation

Install HexStack CLI globally:

```sh
cargo install hexstack
```

Or add individual components to an existing project:

```sh
cargo add ripress --features with-wynd
cargo add wynd --features with-ripress
cargo add tokio --features macros,rt-multi-thread
```

## CLI Usage

### Create a New Project

```sh
# Full-stack HTTP + WebSocket application
hexstack new my-app --template full

# HTTP-only application
hexstack new my-api --template http

# WebSocket-only application
hexstack new my-ws --template websocket
```

### Available Templates

* **`http`** - Ripress HTTP server only
* **`websocket`** - Wynd WebSocket server only
* **`full`** - Ripress + Wynd integrated server *(default)*

> Templates currently scaffold the HTTP and WebSocket layers (Ripress + Wynd). Lume-based data-layer scaffolding isn't included yet, since Lume itself is still experimental.

## Examples

### Full-Stack Application

```rust
use ripress::{app::App, types::RouterFns};
use wynd::wynd::{Wynd, WithRipress};

#[tokio::main]
async fn main() {
    let mut wynd: Wynd<WithRipress> = Wynd::new();
    let mut app = App::new();

    wynd.on_connection(|conn| async move {
        conn.on_text(|event, handle| async move {
            let _ = handle.send_text(&format!("Echo: {}", event.data)).await;
        });
    });

    app.get("/", |_, res| async move {
        res.ok().text("Welcome to HexStack!")
    });

    app.get("/api/health", |_, res| async move {
        res.ok().json(json!({"status": "healthy"}))
    });

    app.use_wynd("/ws", wynd.handler());

    app.listen(3000, || {
        println!("HexStack server running on http://localhost:3000");
        println!("WebSocket available at ws://localhost:3000/ws");
    })
    .await;
}
```

### HTTP-Only Application

```rust
use ripress::{app::App, types::RouterFns};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.get("/", |_, res| async move {
        res.ok().json(json!({"message": "Hello from HexStack!"}))
    });

    app.use_cors(None)
        .use_rate_limiter(None);

    app.listen(3000, || {
        println!("HexStack API running on http://localhost:3000");
    })
    .await;
}
```

### WebSocket-Only Application

```rust
use wynd::wynd::{Wynd, Standalone};

#[tokio::main]
async fn main() {
    let mut wynd: Wynd<Standalone> = Wynd::new();

    wynd.on_connection(|conn| async move {
        println!("Client connected");

        conn.on_text(|event, handle| async move {
            println!("Received: {}", event.data);
            let _ = handle.send_text("Message received!").await;
        });

        conn.on_binary(|event, handle| async move {
            let _ = handle.send_binary(event.data.clone()).await;
        });
    });

    wynd.listen(3000, || {
        println!("WebSocket server running on ws://localhost:3000");
    })
    .await
    .unwrap();
}
```

View more examples in the [Examples](./examples/) directory.

## Get Started

Ready to build something amazing? Get started in 30 seconds:

```sh
# Install HexStack CLI
cargo install hexstack

# Create a new full-stack app
hexstack new my-app

# Navigate and run
cd my-app
cargo run
```

Your server will be running on `http://localhost:3000` with WebSocket support at `ws://localhost:3000/ws`.

## Documentation

* [Getting Started Guide](./docs/getting-started.md)
* [CLI Reference](./docs/cli-reference.md)
* [HTTP Server Guide](./docs/guides/http-server.md)
* [WebSocket Guide](./docs/guides/websocket.md)
* [Deployment Guide](./docs/guides/deployment.md)
* [API Reference](./docs/api-reference/)

## Ecosystem

HexStack is built on top of these libraries:

### [Ripress](https://github.com/guru901/ripress) - HTTP Server

* Express.js-inspired API
* Built-in middleware ecosystem
* HTTP/2 support
* Feature-complete and maintained

### [Wynd](https://github.com/guru901/wynd) - WebSocket Server

* Event-driven async API
* Automatic connection management
* Type-safe message handling
* Seamless HTTP integration
* Feature-complete and maintained

### [Lume](https://github.com/guru901/lume) - ORM *(Experimental)*

* Type-safe queries
* Schema-first approach
* Migration system
* Relationship handling
* **Status**: Development is currently paused, but the project is not abandoned and is planned to be developed further.

## Performance

HexStack delivers strong performance across the stack:

* **HTTP**: 97% of Actix-Web performance
* **WebSocket**: Full async/concurrent connections
* **Memory**: Efficient connection and resource management
* **Throughput**: 10x faster than Express.js + Socket.io equivalents

## Contributing

HexStack is not currently undergoing active feature development, but the project is maintained.

If you find something that needs attention or have an improvement that makes sense for the project, feel free to open an issue or submit a pull request.

The lack of frequent commits is intentional. HexStack has reached a stable point where there isn't much that needs to be changed right now.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

---

*HexStack v0.3.0 — Feature-complete for its current scope and maintained.*
