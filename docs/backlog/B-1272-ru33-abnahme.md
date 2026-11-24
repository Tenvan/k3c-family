# B-1272 · Abnahme RU33: Fensterlauf sim run, Sim-, Balance- und Gen-Läufe

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU33
- **Projekt:** RSU
- **Erstellt:** 2026-11-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint RU33 (Läufe im Client) ist umgesetzt; Sim, Balance, Gen und `check:race` laufen wegen der Sim-Sperre erst hier (B-742).

## Ziel

RU33 ist abgenommen: der Fensterlauf und die gesperrten Läufe sind einmal gelaufen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

1. `k3c-game sim run --window --speed 8 --input lauf.json --data data` (Gerät: PC): Lauf sichtbar, Kamera folgt dem Monarchen, bei `--speed 64` nicht zurückbleibend, Fenster bleibt am Ende kurz stehen, danach Zeile `result` und Exit 0; Fenster vorzeitig schließen ergibt `aborted` und Exit 130. 2. Nach Widerruf der Sim-Sperre: `check:race`, `gen:paper` samt Grafik-Golden und ein längerer Lauf `sim run` gegen `k3c-sim` (gleiche Zeile). 3. Der Test `run_genau_die_grenze_geht` (100000 Ticks, `#[ignore = "lang"]`) läuft einmal.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Fensterlauf von 🧑 am PC bestätigt.
- **AC-02** Die gesperrten Läufe sind gelaufen oder mit Grund verschoben.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
