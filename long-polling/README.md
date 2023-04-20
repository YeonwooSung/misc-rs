# Long Polling in Rust

This is a simple example of web-based chat application server with long polling.

## Tricks

1. Use `tokio::time::sleep` to simulate a long-running task.

2. Use `".layer(AddExtensionLayer::new(ctx));"` for injecting the ServerContext, so that we can use the same instance of ChatService across all the routes.

3. Use `"after.unwrap_or(Uuid::nil())"`, which returns a "zero" UUID (00000000-0000-0000-0000-000000000000), with WHERE id > $1 to return all the messages if after is None.
