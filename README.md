# Respite

A customizable restaurant ordering app.

- Sign up and login to create a restaurant.
- Add items to the restaurant menu, with optional image and description.
- Customers can now order through the app.

### Setup

Build each service:

```
docker build --build-arg service=auth -t auth .
docker build --build-arg service=store -t store .
docker build --build-arg service=menu -t menu .
docker build --build-arg service=waiter -t waiter .
```

Run each service with environment variables:

```
docker run -d --expose=6030 \
  -e auth_uri="127.0.0.1:6030" \
  -e db_uri="postgres://username:password@localhost:5432/respite" \
  -e db_pool_size="3" \
  -e jwt_secret="blah" \
  auth
docker run -d --expose=6060 \
  -e store_uri="127.0.0.1:6060" \
  -e db_uri="postgres://username:password@localhost:5432/respite" \
  -e db_pool_size="3" \
  store
docker run -d --expose=6090 \
  -e menu_uri="127.0.0.1:6090" \
  -e db_uri="postgres://username:password@localhost:5432/respite" \
  -e db_pool_size="3" \
  menu
docker run -d --expose=6300 \
  -e waiter_uri="127.0.0.1:6300" \
  -e db_uri="postgres://username:password@localhost:5432/respite" \
  -e db_pool_size="3" \
  waiter
```

Run web server:

```
cd web
npm run build
npm run start
```
