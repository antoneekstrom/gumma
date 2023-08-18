# GUMMA-AUTH
An entity provider that enables authentication and authorization for other services.

# Behaviour
The service provides an API for querying identity data, and a standard OAuth2 solution for authentication.

# Deployment

## Configuration
Environment variables:


## Building & running
The service can be built and deployed with a docker container.

```
docker build -t gumma-auth -d .
```

# Development
You will need to have rust and cargo installed in order to build the service locally, alternatively you can use docker to deploy.

## Testing
You can run the tests using cargo.

```
cargo test
```

## Building & running
Or built and installed locally with cargo using the following command.

```
cargo install --path .
```