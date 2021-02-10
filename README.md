# jwtool

This is a small tool to convert JSON Web Tokens from and to JSON.

```console
$ cat test/example.jwt
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

$ jwtool decode test/example.jwt
{
  "iat": 1516239022,
  "name": "John Doe",
  "sub": "1234567890"
}

$ jwtool encode test/example.json
eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

Decoding is done **without any signature verification/validations**.
Headers are not displayed.

## Installation

You may install `jwtool` locally by running

```console
$ cargo install https://github.com/vivienm/jwtool.git
```

### Autocompletion

To enable autocompletion in Bash, run

```console
$ source <(jwtool completion bash)
```
