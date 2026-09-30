# Heimnetz-Server K3C (SP03): Spiel-Build und Go-Server in einem kleinen Image für amd64 und arm64 (Raspberry Pi).
# Bauen:   docker buildx build --platform linux/amd64,linux/arm64 -t k3c-server .
# Starten: docker compose up -d   (Port 8080, Spielstände und Berichte im Volume unter /data)

# Spiel-Build auf der Build-Plattform (ein Build für alle Zielplattformen)
FROM --platform=$BUILDPLATFORM node:22-alpine AS web
WORKDIR /src
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
RUN npx tsc --noEmit && npx vite build

# Go-Server für die Zielplattform, ohne cgo (statisch)
FROM --platform=$BUILDPLATFORM golang:1.27-alpine AS server
ARG TARGETOS
ARG TARGETARCH
ARG VERSION=dev
WORKDIR /src
COPY go.mod ./
COPY data ./data
COPY engine ./engine
COPY cmd ./cmd
RUN CGO_ENABLED=0 GOOS=$TARGETOS GOARCH=$TARGETARCH \
    go build -trimpath -ldflags "-s -w -X main.version=$VERSION" -o /out/k3c-server ./cmd/k3c-server \
 && mkdir -p /out/data/saves /out/data/reports

# Laufzeit ohne Shell, als nonroot (UID 65532); /data gehört ihm, damit ein neues Volume beschreibbar ist
FROM gcr.io/distroless/static-debian12:nonroot
WORKDIR /app
COPY --from=web /src/dist ./dist
COPY --from=server /out/k3c-server ./k3c-server
COPY --from=server --chown=65532:65532 /out/data /data
ENV K3C_DIST=/app/dist K3C_SAVES_DIR=/data/saves K3C_REPORTS_DIR=/data/reports K3C_HTTP_PORT=8080
VOLUME ["/data"]
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s CMD ["/app/k3c-server", "-health"]
ENTRYPOINT ["/app/k3c-server"]
