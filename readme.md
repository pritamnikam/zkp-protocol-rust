# Zero-Knowledge Proof Authentication

## Docker

```
# Build images
$ docker-compose build zkpserver

# Run the server
$ docker-compose run --rm zkpserver
root@e84736012f9a:/zkp-server# cargo run --bin server --release

# Run the client
$ docker container ls
$ docker exec -it e84736012f9a /bin/bash
root@e84736012f9a:/zkp-server# cargo run --bin client --release
```
