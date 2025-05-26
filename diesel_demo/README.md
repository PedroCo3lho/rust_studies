# Diesel Demo
Built with:
- Diesel
- Dotenvy 
- PostgresSQL

## How to run 

1. Configure your Database URL in the .env file

```bash
mv .env.example/ .env
```

2. Setup the project through the Diesel CLI 

```bash
diesel setup
```

3. Run the migrations

```bash
diesel migration run 
```

4. Interact whith the project

```bash 
cargo run --bin write_post
cargo run --bin publish_post 1
cargo run --bin get_post 1
cargo run --bin show_posts
```