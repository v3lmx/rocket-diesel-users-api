# Production deployment

## Dockerizing

The recommended deployemnt method is using a docker-compose file.
The production docker image can be built in the CI of the project, and published to your private docker repository. Then just use this image alongside your database docker image.

### Bulding the image

Using a base of a lightweight image, such as `alpine:latest`, copy the api executable, as well as the configuration and keys. Set the entrypoint to launch the executable.

## Architecture

Docker containers can be deployed to a number of places, but the simplest will be in a virtual machine on your network, or in your cloud provider (most allow docker deployments).

A reverse proxy has been added compared to the development setup, to act as an ingress to the API. It will act as a passthrough to the API, keeping the session secure up to the API itself. 

## Authentication

Users authentify to the API using certificates. This means users without a certificate will not be able to use any functionnality. Instructions to generate certificates can be found below.

Be mindful of secrets: passwords and private keys must not be saved to the version control system (git in this case). The provided keys and passwords are here for development purposes and must not be used in production.

Depending on your security requirements, you should consider using certificate authentication for the connection to your database as well.

### Generate keys and certificates

Here are the comnmands to generate keys and certificates :

#### Certificate Authority

```
# Generate a key
openssl genpkey -algorithm prime256v1 > ca.key

# Generate a CSR (certificate signing request):
openssl req -new -out ca.csr -key ca.key

# Generate the certificate
openssl x509 -req -days 3650 -in ca.csr -signkey ca.key -out ca.crt
```

The `api.csr` file can be deleted, and the certificate and private key are in the files `api.crt` and `api.key` respectively.

#### Generate a certificate for the api

```
# Generate a key
openssl genpkey -algorithm prime256v1 > api.key

# Generate a CSR (certificate signing request):
# The api.cnf file can be found in ./config/certs
openssl req -new -out api.csr -key api.key -config api.cnf

# Generate the certificate
openssl x509 -req -days 3650 -in api.csr -CAkey ca.key -CA ca.crt -out api.crt -extfile api.cnf -extensions v3_req
```

#### Generate a certificate for a client

```
# Generate a key
openssl genpkey -algorithm prime256v1 > client.key

# Generate a CSR (certificate signing request):
# The client.cnf file can be found in ./config/certs
openssl req -new -out client.csr -key client.key -config client.cnf

# Generate the certificate
openssl x509 -req -days 3650 -in client.csr -CAkey ca.key -CA ca.crt -out client.crt -extfile client.cnf -extensions v3_req
```

## Logging

Logs are in json format to be easily parsed and treated. A log line looks like this :

```json
{"timestamp":"2023-11-13T20:56:29.956098Z","level":"WARN","fields":{"message":"Rocket has launched from https://0.0.0.0:8000","log.target":"rocket::launch","log.module_path":"rocket::rocket","log.file":"/usr/local/cargo/registry/src/index.crates.io-6f17d22bba15001f/rocket-0.5.0-rc.4/src/rocket.rs","log.line":668},"target":"rocket::launch"}
```