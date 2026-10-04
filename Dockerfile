# Heimnetz-Server K3C (SP03): Spiel-Build und Go-Server in einem kleinen Image für amd64 und arm64 (Raspberry Pi).
# Bauen:   docker buildx build --platform linux/amd64,linux/arm64 -t k3c-server .
# Starten: docker compose up -d   (Port 8080, Spielstände und Berichte im Volume unter /data)

# Figuren-Atlas packen (tools/atlas, nur Standardbibliothek; B-196): public/atlas/ ist nicht eingecheckt
FROM --platform=$BUILDPLATFORM golang:1.27-alpine AS atlas
WORKDIR /src
COPY go.mod go.sum ./
COPY tools/atlas ./tools/atlas
COPY data/sprites.json ./data/sprites.json
COPY public/sprites ./public/sprites
RUN go run ./tools/atlas

# Spiel-Build auf der Build-Plattform (ein Build für alle Zielplattformen)
FROM --platform=$BUILDPLATFORM node:22-alpine AS web
WORKDIR /src
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
COPY --from=atlas /src/public/atlas ./public/atlas
# Version des Clients (Landingpage, Lobby, Debug-Overlay); .git fehlt im Image, deshalb als Build-Argument
ARG VERSION=dev
RUN npx tsc --noEmit && K3C_VERSION=$VERSION npx vite build

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
ENV NO_COLOR=1 K3C_DIST=/app/dist K3C_SAVES_DIR=/data/saves K3C_REPORTS_DIR=/data/reports K3C_HTTP_PORT=8080
VOLUME ["/data"]
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s CMD ["/app/k3c-server", "-health"]
ENTRYPOINT ["/app/k3c-server"]
