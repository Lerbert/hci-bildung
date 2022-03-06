[![Build Status](https://github.com/Lerbert/hci-bildung/actions/workflows/build_status.yml/badge.svg)](https://github.com/Lerbert/hci-bildung/actions/workflows/build_status.yml)

# HCI Bildung

## Project setup

This project relies on enriching static pages served from the rust backend with an vue app for the editor component. To do so, we use webpack to build a production ready version of the vue app, which our backend then picks up as a template.

### Setup with Docker Compose

1. Create a `.env` file at the repository root. Have a look at `.env.example` to see how this file might look like
You can generate a suitable secret key with `openssl rand -base64 32`.

2. To start the backend in the background run

```bash
docker compose up -d
```

If you want to rebuild the docker images add `--build`

3. You can stop the backend with

```bash
docker compose down
```

### Setup without Docker Compose

As an alternative to using Docker Compose you can setup your development environment as follows.

#### Setting up the database

1. Create a new PostgreSQL database

2. Update `Rocket.toml` with your database URL

```toml
[global.databases.hci_bildung]
url = "<your URL here>"
```

3. The database will be initialized by the Diesel migrations, which run automatically when the backend starts. If you want to use the Diesel CLI, you have to set `DATABASE_URL` in your `.env` file.

#### Configuring Rocket

1. For production builds: Set a secret key for encrypted cookies. This can be done in `Rocket.toml` or using the environment variable `ROCKET_SECRET_KEY`

```toml
[release]
secret_key="<your secret key here>"
```

You can generate a suitable key with `openssl rand -base64 32`.

2. Further configuration can be done via `Rocket.toml` or environment variables as described in the [Rocket docs](https://rocket.rs/v0.5-rc/guide/configuration/#configuration)

#### Building the frontend

1. Navigate to the `vue` directory

```bash
cd vue
```

2. Install node dependencies

```bash
npm install
```

3. Build vue app

```bash
npm run build
```

This should produce the directory `vue_dist` at the root of the repository.
The directory contains the file `sheet.html.tera`, which our backend loads as template, and the directory `vue` containing the assets.
After starting the backend, the `vue` directory is available at `/vue` enabling the vue app to load its assets correctly.

#### Building and running the backend

1. Navigate back to the repository root, so if you previously built the frontend do

```bash
cd ..
```

2. Launch the backend with

```
cargo run
```

The backend serves additional assets from the `assets` directoy at `/assets`.

## Development of the vue app

All commands in this section have to be executed from the `vue` directory.

For more convenient development of the vue app run

```bash
npm run serve
```

This will start a development server with hot-reloads for the vue app only.

To lint and fix files run

```bash
npm run lint
```
