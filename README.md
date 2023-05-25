Based on the following Youtube video:
https://www.youtube.com/watch?v=XZtlD_m59sM

## Command List

---

### Run src and watch for changes
```bash
cargo watch -q -c -w "src/" -x "run"
```

### Run tests and watch for changes
```bash
cargo watch -q -c -w "tests/" -x "test quick_dev -- --nocapture"
```

