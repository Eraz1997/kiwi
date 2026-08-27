# Web Builder Stage (node:26.7.0-trixie)

from node@sha256:2baf63043c99f4e98ed174ac428bf87f0f9ac5bf49194d60ade5acad24a1efda as web-builder

    copy web /src
    workdir /src
    run npm install -g pnpm
    run pnpm install --frozen-lockfile
    run pnpm vite build --outDir /dist

# Backend Builder Stage (rust:1.98.0-trixie)

from rust@sha256:bb3b8b0b0fa67da87b913ae57e8b3a860d6988e77eeea4aa63b496d219531bd3 as backend-builder

    copy backend /src
    workdir /src
    run cargo install --path . --root /dist

# Runtime (debian:trixie-20260824-slim)

from debian@sha256:d7e12182ce18b85b93007c1dedf31f2d29e01ccf3182cc4017c709b6259bc132 as runtime

    ## Install System Dependencies

    run apt update
    run apt install -y curl

    ## Install Artifacts

    copy --from=web-builder /dist /static
    copy --from=backend-builder /dist /app

    ## Set Up Health Checks

    healthcheck --interval=10s --timeout=1m --retries=5 --start-interval=20s cmd curl -k https://status.kiwi-local.com/api/health

    ## Runtime Command

    cmd /app/bin/kiwi-api \
        --config-folder-path /config \
        --host 0.0.0.0 \
        --lets-encrypt-environment production \
        --log-level info \
        --port 443 \
        --static-files-path /static
