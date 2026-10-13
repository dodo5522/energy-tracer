# Running Migrator CLI

Do the following instructions after running `cargo install sea-orm-cli`.

- Generate a new migration file
    ```sh
    sea-orm-cli migrate -d ./crates/infra-db-migration generate 'create junction table for sites and devices'
    ```
- Apply all pending migrations
    ```sh
    cargo run -p infra-db-migration
    ```
    ```sh
    cargo run -p infra-db-migration -- up
    ```
- Apply first 10 pending migrations
    ```sh
    cargo run -p infra-db-migration -- up -n 10
    ```
- Rollback last applied migrations
    ```sh
    cargo run -p infra-db-migration -- down
    ```
- Rollback last 10 applied migrations
    ```sh
    cargo run -p infra-db-migration -- down -n 10
    ```
- Drop all tables from the database, then reapply all migrations
    ```sh
    cargo run -p infra-db-migration -- fresh
    ```
- Rollback all applied migrations, then reapply all migrations
    ```sh
    cargo run -p infra-db-migration -- refresh
    ```
- Rollback all applied migrations
    ```sh
    cargo run -p infra-db-migration -- reset
    ```
- Check the status of all migrations
    ```sh
    cargo run -p infra-db-migration -- status
    ```
