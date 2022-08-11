# monit

Monitor website response time.

## How to use

### USAGE

```
monit [OPTIONS] <URL>
```

### ARGS

```
<URL>    URL to be monitored for response time
```

### OPTIONS

```
-b, --bolder <BOLDER SECONDS>
            Bolder seconds for alert [default: 3]

    -f, --file <FILE PATH>
            File path for output

    -h, --help
            Print help information

    -i, --interval <INTERVAL SECONDS>
            Interval seconds [default: 30]

    -o, --output <OUTPUT TYPE>
            Output type [default: text] [possible values: csv, json, text]

    -V, --version
            Print version information
```

### Examples

```
$ monit http://127.0.0.1
2022-08-08 22:03:54 http://127.0.0.1 200 OK 0.491
2022-08-08 22:04:24 http://127.0.0.1 200 OK 1.311
2022-08-08 22:04:56 http://127.0.0.1 200 OK 1.331
```

```
$ monit http://127.0.0.1 --output json
{"datetime": "2022-08-11 18:48:34","url: "http://127.0.0.1","statusCode": "200 OK","responseTime": "0.758"}
{"datetime": "2022-08-11 18:48:35","url: "http://127.0.0.1","statusCode": "200 OK","responseTime": "1.057"}
{"datetime": "2022-08-11 18:48:38","url: "http://127.0.0.1","statusCode": "200 OK","responseTime": "0.443"}
```
