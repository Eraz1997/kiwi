# Kiwi Development Guidelines 👨‍💻

## Folder structure 🪛

- `.github` contains CI workflows to build Kiwi
- `backend` contains the code of the REST API backend, written in Rust
- `ci` contains a composite Github Action to deploy services to a Kiwi instance
- `web` contains the code of the web app, written in TypeScript using SolidJS and Park UI

## Setup 🪛

### General 🥝

You will access your local server through a test domain, `kiwi-local.com`. To make it point to your local machine, add the following entries to your `/etc/hosts` file:

```
# Kiwi development environment
127.0.0.1       kiwi-local.com                  # main domain
127.0.0.1       auth.kiwi-local.com             # auth service
127.0.0.1       admin.kiwi-local.com            # admin service
127.0.0.1       ci.kiwi-local.com               # ci service
127.0.0.1       status.kiwi-local.com           # status service
127.0.0.1       test-service-1.kiwi-local.com   # test service
```

You can add other lines like `127.0.0.1       test-service-2.kiwi-local.com   # test service`, they will be useful to test registered services within Kiwi.

### Web 🕷️

1. Install `fnm` ([guide](https://github.com/Schniz/fnm))

1. Install the latest Node.js version:

   ```sh
   fnm install --latest --corepack-enabled
   fnm use <INSTALLED_VERSION>
   ```

1. Install dependencies with `pnpm install`

### Backend 🎒

1. Install Docker ([guide](https://docs.docker.com/engine/install/))

1. [Install Rust](https://www.rust-lang.org/tools/install)

## Run Locally 🧸

### Web 🕷️

```sh
pnpm dev
```

If you run the frontend only, you can access it at `http://<service>.kiwi-local.com:3000/<path>`.

### Backend 🎒

```sh
cargo run
```

The backend service is available at `https://<service>.kiwi-local.com:5000/<path>`. Paths starting with `/api` will be forwarded to API handlers, while the others will be forwarded to the frontend server, if any is running.

Please note that only HTTPS is supported by the backend, and any attempt to access an `http://` URL will give invalid response. Your browser will initially show a warning due to untrusted certificates, as the backend generates dummy ones if it doesn't find some.

## Lint and Format 🧽

### Web 🕷️

```sh
pnpm lint
```

### Backend 🎒

```sh
cargo fmt
cargo clippy -- --deny warnings
```
