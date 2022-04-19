# Rust Web App - 1/3 - Data Access
YouTube tutorials: 

- Part 1 (database / model access) - https://youtu.be/VIig9IcQ-w8
- Part 2 (web / warp filters) - https://youtu.be/plKzUo8F6Mg
- Part 3 (frontend / #FrameworkLess) - https://youtu.be/DkR0tCBPqYc

Code:
https://github.com/jeremychone-channel/rust-todomvc


Install cargo-watch
```
cargo install cargo-watch
```

## Dev Test 

```sh
# Test for db
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'

# Test for model
cargo watch -q -c -w src/ -x 'test model_ -- --test-threads=1 --nocapture'

# Test for web
cargo watch -q -c -w src/ -x 'test web_ -- --test-threads=1 --nocapture'
```

## Dev Web

```sh
cargo watch -q -c -w src/ -x 'run -- ../frontend/web-folder'
```

## DB

```sh
# Start the database
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# optional psql (other terminal) 
docker exec -it -u postgres pg psql
```