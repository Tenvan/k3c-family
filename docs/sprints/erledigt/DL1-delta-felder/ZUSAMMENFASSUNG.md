# DL1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Delta überträgt verschwundene Felder
- **Projekt:** –

## Ziel

Nach diesem Sprint entfernt der Client jedes Feld, das der Server nicht mehr schickt, und W4.3a kann weiterlaufen. Am Ende sichtbar: `task check:go` grün, auch mit dem Stand von `wip/w4.3a-krieger`.

## Ergebnis

2026-10-05, Review DL1.2 (leicht, nur schwere Befunde): keine schweren Befunde. tsc, oxlint (0 Fehler), Vitest (1337), `go test`, golangci-lint (0 issues) grün. `null` bleibt Wert, `events` nie in `unset`, Server und Client entfernen dieselben Felder; AC-01 bis AC-04 mit Tests und Protokolltext belegt (`delta_test.go`, `clientDelta.test.ts`, `docs/protocol.md`). AC-03: Der Test trifft die Burg selbst alle 600 Ticks, weil sie mit W4.3a in keinem Golden-Lauf fällt; `castle` läuft weiter über `deltaOf` und `apply`. B-297 archiviert, W4.3a ist entsperrt. Version: v0.12.1 vorgeschlagen (Fehlerkorre …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Server nennt verschwundene Felder in `unset`, `apply` ergibt genau den neuen Zustand (B-29 … |
| AC-02 | – | Client entfernt die Felder aus `unset` (B-297/AC-02). |
| AC-03 | – | Delta-Test prüft `castle` weiter, auch mit dem Stand von W4.3a (B-297/AC-03). |
| AC-04 | – | Protokoll beschrieben, `task check` und `task check:go` grün (B-297/AC-04). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-297 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 – | Das Delta überträgt, dass ein Feld aus dem Zustand verschwindet |
