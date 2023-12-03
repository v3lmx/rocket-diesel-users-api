# Users API

An simple API to manage users using Rust, Rocket, and Diesel.

> Disclaimer: although this README provides production instructions, this app should not be used in production as it is without
first reviewing your security and functionnality requirements. The purpose of this code is to serve as a proof of
concept, not a finished product.

## Production 

See [Production.md](./Production.md) for production instructions.

## Development

To start the api and the database :

```
docker-compose -f docker-compose-dev.yml up
```

The api can then be called at https://localhost/api/v1/. Examples are available below.

To scratch the database :

```
docker container rm <db-container>
docker volume rm -f <db-volume>
```

## Usage examples

Create a user :

```
curl --request POST \
  --url https://localhost:8000/api/v1/user \
  --header 'Content-Type: application/json' \
  --data '{
	"first_name": "Spongebob",
	"last_name": "Squarepants",
	"email": "sponge@mail.com",
	"password_hash": "$2a$10$WowcJIP00VXf6HhTeeoOrudjFlyVRKqcMWaSWaJV.NoqRpntx0HQC",
    "role": "User"
    }' \
  --cert ./config/certs/client-dev.crt \
  --key ./config/certs/client-dev.key \
  --cacert ./config/certs/ca-dev.crt
```

Get all users :

```
curl --request GET \
  --url https://localhost:8000/api/v1/user \
  --cert ./config/certs/client-dev.crt \
  --key ./config/certs/client-dev.key \
  --cacert ./config/certs/ca-dev.crt
```

Get a single user :

```
export UUID='<UUID>'
```

```
curl --request GET \
  --url https://localhost:8000/api/v1/user/${UUID} \
  --cert ./config/certs/client-dev.crt \
  --key ./config/certs/client-dev.key \
  --cacert ./config/certs/ca-dev.crt
```

Update a user :

> All fields are optional.

```
export UUID='<UUID>'
```

```
curl --request PUT \
  --url https://localhost:8000/api/v1/user/${UUID} \
  --header 'Content-Type: application/json' \
  --data '{
	"email": "sponge@protonmail.ch",
    "role": "Admin"
    }' \
  --cert ./config/certs/client-dev.crt \
  --key ./config/certs/client-dev.key \
  --cacert ./config/certs/ca-dev.crt
```

Delete a user :

```
export UUID='<UUID>'
```

```
curl --request DELETE \
  --url https://localhost:8000/api/v1/user/${UUID} \
  --cert ./config/certs/client-dev.crt \
  --key ./config/certs/client-dev.key \
  --cacert ./config/certs/ca-dev.crt
```

## Architecture

The app is divided in a few modules.

`routes.rs` contains the HTTP routes exposed by the API.

`user.rs` contains the user structures manipulated in the API.

The `db` module contains the abstraction `UserDao` that defines all the functions needed to interact with the database, and the implementation for the `Connection` type, that is passed by `Rocket` to our routes.

Changing storage can be achieved by implementing the `UserDao` trait, and other minor changes. The database is not completly decoupled from the routes, as we can leverage `Rocket`'s databse management, resulting in simpler code, but this can be refactored as the application grows.

`tracing_fairing.rs` contains code taken from another repository, and allows integration between `Rocket` and `tracing` for automated logs.