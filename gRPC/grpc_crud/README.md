# Tonic gRPC CRUD Project

Steps to run the project: 

1. Set up a docker PostgreSQL image:

```bash 
docker run --name crud-postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres
psql -h localhost -U postgres -d postgres # to verify the connection, with pwd  = postgres
```

> Look at the [postgres docker hub](https://hub.docker.com/_/postgres/) to know more about the DB setup

2. Diesel and DB setup:

- Prerequisite: `diesel-cli`, run `cargo binstall diesel_cli`

```bash 
diesel setup 
diesel migration run
```

> Additional information and troubleshooting at: https://diesel.rs/guides/getting-started.html

