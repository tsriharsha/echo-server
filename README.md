# Echo Server

Simple rust based echo server. It parrots your headers back to you

## Usage

```bash
$ cargo run --bin echo-server
```

```bash
$ curl --request GET -sL \
     --url 'http://localhost:3000/echo'
```