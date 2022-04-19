


## Cargo watch
Install cargo-watch
```
cargo install cargo-watch
```

moniter folder src/
```
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'
```



## Run tests has specific name prefix
```
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'
```


## Put test in a seperate folder

```
// region:    Test
#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
// endregion: Test
```


## Test skeleton
```
  // -- FIXTURE
	
	// -- ACTION
	
	// -- CHECK - deleted item
	
	// -- CHECK - list
```