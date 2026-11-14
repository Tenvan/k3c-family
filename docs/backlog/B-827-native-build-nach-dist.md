# B-827 · task native:game:build legt das komplette Spiel nach dist/k3c-game und kopiert geänderte Grafik neu

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG20
- **Projekt:** NAT
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-13 (Dialog)

## Ausgangslage

Abnahme NG14.5 (Ralf, 2026-11-13): Mit dem Build ließen sich die Grafiken nicht neu ins Ziel kopieren, und der Build soll das komplette Spiel nach `dist` legen. Ursache (belegt): Tasks eingebaute Shell nutzt unter Windows ein eigenes `cp` ohne `-u` und ohne Zusammenführen vorhandener Ordner; `game:build` baute nur die EXE.

## Ziel

`task native:game:build` baut EXE und Grafik und bringt das komplette Spiel nach `dist/k3c-game/`; geänderte Dateien werden ersetzt, unveränderte bleiben.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `game:exe` baut nur die EXE (Abhängigkeit von `game`, `shots`, `e2e`, `measure`).
- `game:build` = `game:exe` + `gen:paper` + Abgleich nach `dist/k3c-game` (EXE, geladene Grafik inkl. Menü, Ton, Schrift, Rig-Teile, `LIZENZEN.txt`, Messskript).
- Kopieren über `native/k3c-game/cmd/sync` (Go): nur bei abweichender Größe oder Zeit.
- `game:paket` leert `dist/k3c-game`, ruft `game:build`, dann die Probe.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Tasks nur über `task`.

- **Bereich:** `native/Taskfile.yml`, `native/k3c-game/cmd/sync/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Test `cmd/sync`: Ordner werden zusammengeführt, nur Geändertes kopiert.
- **AC-02** `task native:game:build` zweimal: zweiter Lauf kopiert 0 Dateien; nach geänderter Grafik genau diese.
- **AC-03** `dist/k3c-game/k3c-game.exe` startet mit Grafik (Sichtprüfung 🧑).

## Offene Fragen

keine

## Notizen

Vermutet: Der Web-Build (`task build`, Vite) leert `dist/` und löscht damit auch `dist/k3c-game`, bis B-823.

NG20.1 (2026-11-14, Claude): Umsetzung lag bereits mit #575 in main (game:exe, game:build, game:dist mit cmd/sync, game:paket). Geprüft:
- AC-01 `go test ./cmd/sync/` grün.
- AC-02 `task native:game:build` zweiter Lauf: „0 Dateien aktualisiert“; nach `touch` einer Portal-Grafik: „1 Dateien aktualisiert“. Spiel in `dist/k3c-game` (50 MB).
- AC-03 Sichtprüfung übernimmt 🧑.
