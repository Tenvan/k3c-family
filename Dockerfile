# Heimnetz-Server K3C (SP03): Spiel-Build und Go-Server in einem kleinen Image für amd64 und arm64 (Raspberry Pi).
# Bauen:   docker buildx build --platform linux/amd64,linux/arm64 -t k3c-server .
# Starten: docker compose up -d   (Port 8080, Spielstände und Berichte im Volume unter /data)

# Spiel-Build auf der Build-Plattform (ein Build für alle Zielplattformen)
FROM --platform=$BUILDPLATFORM node:22-alpine AS web
WORKDIR /src
COPY package.json bun.lock ./
RUN npm i -g bun@1.4.2 && bun install --frozen-lockfile
COPY . .
# Papierschnitt-Grafik rendern: PNG und Manifeste sind nicht eingecheckt (B-737)
RUN bun tools/paper/bin/gen.ts
# Version des Clients (Landingpage, Lobby, Debug-Overlay); .git fehlt im Image, deshalb als Build-Argument
ARG VERSION=dev
RUN bunx tsc --noEmit && K3C_VERSION=$VERSION bunx vite build

# Go-Server für die Zielplattform, ohne cgo (statisch)
FROM --platform=$BUILDPLATFORM golang:1.27-alpine AS server
ARG TARGETOS
ARG TARGETARCH
ARG VERSION=dev
WORKDIR /src
COPY go.mod go.sum ./
COPY data ./data
COPY engine ./engine
COPY cmd ./cmd
COPY tools/assetkit ./tools/assetkit
COPY tools/ruletable ./tools/ruletable
RUN CGO_ENABLED=0 GOOS=$TARGETOS GOARCH=$TARGETARCH \
    go build -trimpath -ldflags "-s -w -X main.version=$VERSION -X main.built=$(date -u +%Y-%m-%dT%H:%M:%SZ)" -o /out/k3c-server ./cmd/k3c-server \
 && CGO_ENABLED=0 GOOS=$TARGETOS GOARCH=$TARGETARCH \
    go build -trimpath -ldflags "-s -w" -o /out/k3c-tui ./cmd/k3c-tui \
 && mkdir -p /out/data/saves /out/data/reports

# Laufzeit ohne Shell, als nonroot (UID 65532); /data gehört ihm, damit ein neues Volume beschreibbar ist
FROM gcr.io/distroless/static-debian12:nonroot
WORKDIR /app
COPY --from=web /src/dist ./dist
COPY --from=server /out/k3c-server ./k3c-server
# Diagnose-TUI: docker exec -it <Container> k3c-tui (K3C_STATUS_TOKEN muss im Container gesetzt sein)
COPY --from=server /out/k3c-tui /usr/local/bin/k3c-tui
COPY --from=server --chown=65532:65532 /out/data /data
# Dev-Mode im Image an (Entwicklungsphase, RX1: Stress-Szenen mit Server brauchen die Dev-Aktion spawn); ausschalten: K3C_DEV=0
ENV K3C_DEV=1NO_COLOR=1 K3C_DIST=/app/dist K3C_SAVES_DIR=/data/saves K3C_REPORTS_DIR=/data/reports K3C_HTTP_PORT=8080
VOLUME ["/data"]
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s CMD ["/app/k3c-server", "-health"]
ENTRYPOINT ["/app/k3c-server"]
