# HCI Bildung

## Project setup

This project relies on enriching static pages served from the rust backend with an vue app for the editor component. To do so, we use webpack to build a production ready version of the vue app, which our backend then picks up as a template.

### Setting up the database

1. Create a new PostgreSQL database

2. Initialize the database using

```bash
psql -d $DB_NAME < db_scripts/setup.sql
```

3. Update `Rocket.toml` with your database URL

```toml
[global.databases.hci_bildung]
url = "<your URL here>"
```

### Building the frontend

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

### Building and running the backend

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
