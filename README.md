# slack-vacation

reveals you're in vacation in slack!!

```
USAGE:
    slack-vacation [OPTIONS] --token <TOKEN>

OPTIONS:
    -b, --back             you're back from vacation
    -d, --date <DATE>      enter date you're in vacation
    -h, --help             Print help information
    -t, --token <TOKEN>    Sets your slack app token
    -V, --version          Print version information
```

## Go To Vacation(Tomorrow)

```
cargo run -- -t token
okaponta -> okaponta(mm/dd休)
```

## Go To Vacation

```
cargo run -- -t token -d date
okaponta -> okaponta(mm/dd休)
```

## Back From Vacation

```
cargo run -- -t token -b
okaponta(mm/dd休) -> okaponta
```

## token

get your token from slack api.

Required
```
users.profile:read
users.profile:write
```
