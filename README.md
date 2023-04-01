# Simple order book

## Description

This is a sample app that shows how to use gRPC to build a simple order book. It is a simple app that
connects to the exchanges and shows the order book for a given pair.

## How to run

First, you need to install the dependencies. You can find the dependencies for each OS below.

### Dependencies for MacOS

```bash
$ brew install protobuf
```

### Dependencies for Ubuntu

```bash
$ sudo apt-get install libprotobuf-dev protobuf-compiler
```

### Running the Server

```bash
$ cargo run --bin krt-server
```

You can interact with the server using [grpcurl](https://github.com/fullstorydev/grpcurl).

The available endpoints are:

#### HealthCheck

Show information about the health of the server. It is used to check if the server is running and connected
to the exchanges. 

You can check the health of the server by calling the following endpoint:

```bash
$ grpcurl -plaintext -import-path ./proto -proto healthcheck.proto '[::1]:50051' healthcheck.HealthCheck/HealthCheck
```

