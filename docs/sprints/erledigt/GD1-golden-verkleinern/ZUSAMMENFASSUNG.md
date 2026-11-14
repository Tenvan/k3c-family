# GD1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Golden-Daten kleiner bei gleicher Prüfschärfe
- **Projekt:** TST

## Ziel

Die Golden-Daten bleiben deutlich unter der Grenze, ohne dass die Go-Tests weniger Abweichungen finden. Am Ende sichtbar: kleinere `testdata/golden/`, `task go:test` grün, Diffs bleiben lesbar.

## Ergebnis

2026-10-25, Claude (GD1.3): `task check` und `task check:go` grün; `check:race` lokal übersprungen (kein C-Compiler). AC-01 erfüllt durch GD1.1/GD1.2: gzip mit textconv-Diffs, 18 756 751 → 760 737 B, vorentschieden über B-624 (KRT). AC-02 durch GD1.2: Inhalt byte-gleich, Abweichungs-Test über den gz-Pfad. arm64 prüft die CI. Die Grenze liegt wieder bei 16 MB; B-620 ist damit hinfällig. Gestapelt auf GEB17 (#482).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | 🧑 hat den Weg gewählt (z. B. Kompression, seltenere Snapshots, nur geänderte Felder); die … |
| AC-02 | geprüft | Eine absichtlich eingebaute Abweichung wird weiter erkannt (Test), `task go:test` amd64 un … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-419 | erledigt | AC-01 erfüllt, AC-02 geprüft | Golden-Daten werden gezielt verkleinert, ohne die Prüfschärfe zu verlieren |
