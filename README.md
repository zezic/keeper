# Keeper

A toy logging service

## Requirements

* [Docker](https://www.docker.com/)
* [Docker Compose](https://docs.docker.com/compose/)

## Running

```shell
cp .env.example .env
docker-compose up
```

## Usage

Using [HTTPie](https://httpie.io/):

```shell
http :3030/log level=INFO message='Service is starting up...'
http :3030/log level=INFO message='Service is shutting down...'
```

```shell
http :3030/log level==INFO timestamp_ge==0 timestamp_le==$(date +%s%N)
```