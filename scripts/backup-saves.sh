#!/bin/sh
# Sichert die Spielstände des nativen Spiels nach <ziel>/k3c-saves-<Datum-Uhrzeit>/ (B-142, B-898, Ziel z. B. USB-Stick).
# Quelle: der Ordner K3C_SAVES_DIR, sonst dist/k3c-game/saves (saves/ neben der EXE des Pakets aus `task build`,
# native/k3c-game/local › DefaultSavesDir). Aufruf im Repo-Stamm. Fehler stehen mit Zeit auf stderr, Exit != 0.
set -eu

now() { date '+%Y-%m-%dT%H:%M:%S'; }
fail() {
	echo "$(now) backup-saves: $*" >&2
	exit 1
}

[ $# -eq 1 ] || fail "Aufruf: backup-saves.sh <ziel>"
[ -d "$1" ] || fail "Ziel $1 fehlt"
[ -w "$1" ] || fail "Ziel $1 nicht beschreibbar"
src="${K3C_SAVES_DIR:-dist/k3c-game/saves}"
[ -d "$src" ] || fail "Quelle $src fehlt"
out="$1/k3c-saves-$(date '+%Y-%m-%d-%H%M%S')"

cp -R "$src" "$out" || fail "Kopieren nach $out gescheitert"
echo "$(now) backup-saves: gesichert nach $out"
