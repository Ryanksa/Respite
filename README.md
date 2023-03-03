# Respite

A customizable restaurant ordering app.

- Sign up and login to create a restaurant.
- Add items to the restaurant menu, with optional image and description.
- Customers can now order through the app.

### Server Setup

Run each service with cargo:

```
cargo run --release --bin api
cargo run --release --bin auth
cargo run --release --bin store
cargo run --release --bin menu
cargo run --release --bin waiter
```

To run with docker, build each service image first:

```
docker build --build-arg service=api -t api .
docker build --build-arg service=auth -t auth .
docker build --build-arg service=store -t store .
docker build --build-arg service=menu -t menu .
docker build --build-arg service=waiter -t waiter .
```

Then run each service with environment variables:

```
docker run -d -p 6000:6000 \
  -e api_uri="0.0.0.0:6000" \
  -e auth_uri="host.docker.internal:6030" \
  -e store_uri="host.docker.internal:6060" \
  -e menu_uri="host.docker.internal:6090" \
  -e waiter_uri="host.docker.internal:6300" \
  -e protocol="http" \
  api

docker run -d -p 6030:6030 \
  -e auth_uri="0.0.0.0:6030" \
  -e db_uri="postgres://username:password@host.docker.internal:5432/respite" \
  -e db_pool_size="3" \
  -e jwt_secret="blah" \
  auth

docker run -d -p 6060:6060 \
  -e store_uri="0.0.0.0:6060" \
  -e db_uri="postgres://username:password@host.docker.internal:5432/respite" \
  -e db_pool_size="3" \
  -e img_path="./img" \
  store

docker run -d -p 6090:6090 \
  -e menu_uri="0.0.0.0:6090" \
  -e db_uri="postgres://username:password@host.docker.internal:5432/respite" \
  -e db_pool_size="3" \
  -e img_path="./img" \
  menu

docker run -d -p 6300:6300 \
  -e waiter_uri="0.0.0.0:6300" \
  -e db_uri="postgres://username:password@host.docker.internal:5432/respite" \
  -e db_pool_size="3" \
  waiter
```

### Proxy Setup

Run the envoy proxy with docker:

```
cd proxy
docker-compose up -d
```

### Web Setup

First, generate the necessary code for protobuf:

```
cd web
npx protoc --ts_out ./src/services/proto --ts_opt long_type_string --ts_opt optimize_code_size --proto_path ../proto api.proto
```

Then, build and run the web server:

```
npm install
npm run build
npm run start
```

To run with docker, build the web image and run the server:

```
docker build -t web .
docker run -d -p 3000:3000 web
```
