# Echo Server

Simple rust based echo server. It parrots your headers back to you

**Hosted service is rate limited to 100 reqs/s!**

**Echo is using ip-api.com and is not being used or sold commercially!**
Learn more about ip-api.com [here](https://ip-api.com/docs/).

## Usage

```bash
$ cargo run --bin echo-server
```

```bash
$ curl --request GET -sL \
     --url 'http://localhost:3000/echo'
```

## Run In Terminal

```bash
curl 'https://echo.tsriharsha.io/echo/ip' \
  -X GET \
  --compressed
```

## Run In Databricks Notebook

```bash
%sh
curl 'https://echo.tsriharsha.io/echo/ip' \
  -X GET \
  --compressed
```