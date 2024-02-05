# livegas

This repository serves as the monorepo for the livegas project which contains three main packages - `web`, `proto`, and `grpc`.

The approach is to have the `proto` package contain the protobuf source files and generate bindings that will both be used by the `web` and `grpc` packages with no additional code needed for communication between the two.

## Web
A Next.js web application that will display a chart of the live gas prices for supported networks.

It uses the protobuf generated bindings from the `proto` package to encode and decode messages and to also communicate with the `grpc` server using `gRPC-Web`.

### Running
```bash
# Install dependencies
bun install

# Generate TS bindings
bun proto

# Start the web server
bun web dev
```

## Proto
A package that contains the protobuf source files and generated bindings for the `web` and `grpc` packages.

It exposes two main RPC methods which are used to subscribe to upcoming blocks and to fetch past blocks.

| Type           | Name                 | Description                                                              |
|----------------|----------------------|--------------------------------------------------------------------------|
| **Service**    | `Gas`                | The main service providing access to gas fee data.                       |
| **RPC Method** | `Subscribe`          | Allows subscription to real-time block updates for a specified network. |
| **RPC Method** | `Blocks`             | Retrieves block data for a range of past blocks.                |

## Grpc
A gRPC server written in Rust using the `tonic` library with a gRPC Web Layer for compatibility with the `web` package. The protobuf bindings are automatically generated from the `proto` package in the `build.rs` file.

It implements the `Gas` service and provides an interface to the `ethers-rs` websocket provider for use in asynchronous and multi-threaded environments, and for storage handling, using `sqlx` for read and write operations on a local SQLite database.

### Database
You can use the `sqlx` CLI to setup the database and run migrations.

```bash
# Install the CLI
cargo install sqlx-cli

# Setup the database
sqlx db create

# Run migrations
sqlx migrate run
```

### Running
```bash
# Start the gRPC server
cargo run
```