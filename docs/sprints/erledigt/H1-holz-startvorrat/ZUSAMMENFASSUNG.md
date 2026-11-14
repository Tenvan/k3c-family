# H1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Holz-Startvorrat
- **Projekt:** –

## Ziel

Eine neue Insel startet mit 100 Holz aus den Daten. Am Ende sichtbar: Go-Tests (neue Insel, geladener Stand, Datenprüfung, Bauen ohne Holzfällen), aktualisierte Golden-Daten, Regel in den Regelwerken.

## Ergebnis

2026-10-03 (H1.2): `task check` und `task check:go` grün, Diff `fc8aa19..origin/main` geprüft, keine schweren Befunde. AC-01 bis AC-06: Nachweise im Ergebnis von H1.1 (`island_start_test.go`, Regel in `docs/rules/materialien-gebaeude.md` und `docs/game-design.md`). Keine behobenen Befunde, keine neuen Tickets. Version: v0.4.0 gesetzt (2026-10-03, auf `80dfb6b`, von 🧑 bestätigt; Minor: neue Insel startet mit 100 Holz).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Eine neue Insel hat Holz 100 und sonst 0 aus `data/hub.json`, deterministisch (B-177/AC-01 … |
| AC-02 | geprüft | Ein geladener Spielstand behält seinen Vorrat (B-177/AC-02). |
| AC-03 | geprüft | Eine Datenprüfung begrenzt den Startvorrat auf das Lager-Maximum (B-177/AC-03). |
| AC-04 | geprüft | Beide Mauern und ein Turm sind ohne Holzfällen baubar (B-177/AC-04). |
| AC-05 | geprüft | Golden-Daten aktualisiert, `task check:go` grün (B-177/AC-05). |
| AC-06 | geprüft | Regel und Verweis stehen in `docs/rules/materialien-gebaeude.md` und `docs/game-design.md` … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-177 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft, AC-06 geprüft | Die Insel startet mit einem Holz-Startvorrat |
