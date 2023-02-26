# Respite

A customizable restaurant ordering app.

- Sign up and login to create a restaurant.
- Add items to the restaurant menu, with optional image and description.
- Customers can now order through the app.

### Docker Setup

Build each service image:

```
docker build --build-arg service=api -t api .
docker build --build-arg service=auth -t auth .
docker build --build-arg service=store -t store .
docker build --build-arg service=menu -t menu .
docker build --build-arg service=waiter -t waiter .
```

Run each service with environment variables:

```
docker run -d -p 6000:6000 \
  -e api_uri="127.0.0.1:6000" \
  -e auth_uri="host.docker.internal:6030" \
  -e store_uri="host.docker.internal:6060" \
  -e menu_uri="host.docker.internal:6090" \
  -e waiter_uri="host.docker.internal:6300" \
  -e protocol="http" \
  api

docker run -d -p 6030:6030 \
  -e auth_uri="127.0.0.1:6030" \
  -e db_uri="postgres://username:password@host.docker.internal:5432/respite" \
  -e db_pool_size="3" \
  -e jwt_secret="blah" \
  auth

docker run -d -p 6060:6060 \
  -e store_uri="127.0.0.1:6060" \
  -e db_uri="postgres://username:password@host.docker.internal:5432/respite" \
  -e db_pool_size="3" \
  -e img_path="./img" \
  store

docker run -d -p 6090:6090 \
  -e menu_uri="127.0.0.1:6090" \
  -e db_uri="postgres://username:password@host.docker.internal:5432/respite" \
  -e db_pool_size="3" \
  -e img_path="./img" \
  menu

docker run -d -p 6300:6300 \
  -e waiter_uri="127.0.0.1:6300" \
  -e db_uri="postgres://username:password@host.docker.internal:5432/respite" \
  -e db_pool_size="3" \
  waiter
```
