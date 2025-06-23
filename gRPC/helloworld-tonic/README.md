## How to run this project?

1. Start the Server

Run:

```bash 
cargo run --bin helloworld-server
```

> **Note:** In this part you can test the server using a gRPC client like Postman, by just setting the `[::1]:50051` as the Server URL, import the `.proto` file and send `{"name": "Tonic"}` as message

2. Client

In another terminal, run:

```bash 
cargo run --bin helloworld-client
```