#!/bin/sh
# Sichert die Spielstände nach <ziel>/k3c-saves-<Datum-Uhrzeit>/ (B-142, Ziel: USB-Stick am Pi).
# Quelle: der Ordner K3C_SAVES_DIR, sonst /data/saves im Compose-Dienst k3c (im Ordner mit compose.yaml aufrufen).
# Fehler stehen mit Zeit auf stderr, Exit != 0; der Spielbetrieb bleibt unberührt.
set -eu

now() { date '+%Y-%m-%dT%H:%M:%S'; }
fail() {
	echo "$(now) backup-saves: $*" >&2
	exit 1
}

[ $# -eq 1 ] || fail "Aufruf: backup-saves.sh <ziel>"
[ -d "$1" ] || fail "Ziel $1 fehlt"
[ -w "$1" ] || fail "Ziel $1 nicht beschreibbar"
out="$1/k3c-saves-$(date '+%Y-%m-%d-%H%M%S')"

if [ -n "${K3C_SAVES_DIR:-}" ]; then
	[ -d "$K3C_SAVES_DIR" ] || fail "Quelle $K3C_SAVES_DIR fehlt"
	cp -R "$K3C_SAVES_DIR" "$out" || fail "Kopieren nach $out gescheitert"
else
	docker compose cp k3c:/data/saves "$out" || fail "docker compose cp nach $out gescheitert"
fi
echo "$(now) backup-saves: gesichert nach $out"
