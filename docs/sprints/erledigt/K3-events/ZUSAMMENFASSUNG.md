# K3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Events Vollmond, Blutmond und Händler-Überfall
- **Projekt:** KMP

## Ziel

Vollmond, Blutmond und Händler-Überfall lösen nach Rhythmus aus und wirken auf Wellen, Schaden und Drops. Am Ende sichtbar: Tests je Event grün, aktualisierte Golden-Daten.

## Ergebnis

2026-10-09, Review K3.3: AC-01 K3.1 (a–g) und K3.2 (`JederVierteBesuch`), AC-02 K3.2 (b–f), AC-03 Golden geprüft ohne Diff (Events nur auf Inseln, kein rng-Aufruf ohne Insel, `rng.json` unverändert), AC-04 und AC-05 K3.2a › Ergebnis. `task check` (MCP) und `task check:go` (Shell, B-372) grün. Keine schweren Befunde; Spielstand v6 mit Fixture und Ladetest, alte Stände laden mit Zähler 0. Leicht, als Ticket: B-379 (Vollmond-Belohnung nur in Stufe 0), B-380 (Blutmond ohne Boss-Flächenangriffe), B-381 (`merchant.raid`/HP im Protokoll, zu K4). Version: v0.15.0 vorgeschlagen (Minor: Vollmond, Blutmo …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Auslöser nach Rhythmus (7., 13., jeder 4. Besuch), Wirkung auf Wellen, Schaden und Drops l … |
| AC-02 | – | Händler-Überfall nur bei Besuch, Belohnung bei Schutz (Test) (B-131/AC-02). |
| AC-03 | geprüft | Golden-Daten aktualisiert, `task check:go` grün (B-131/AC-03). |
| AC-04 | – | Der Händler ist eine angreifbare Figur mit 100 HP, die bei 0 abreist (Test) (B-373/AC-01). |
| AC-05 | – | Besuchszähler je Insel und anwesender Händler überstehen Speichern und Laden, alte Stände … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-131 | erledigt | AC-01 –, AC-02 –, AC-03 geprüft | Vollmond, Blutmond und Händler-Überfall sind als Events umgesetzt |
| B-373 | erledigt | AC-04 –, AC-05 – | Der Händler ist eine angreifbare Figur mit Besuchszähler im Spielstand |
