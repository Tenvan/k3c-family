# NG18 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Natives Spiel: freie Schrift statt Debug-Schrift
- **Projekt:** NAT

## Ziel

Das native Spiel zeichnet mit einer freien Schrift aus Repo oder Go-Modul-Cache, alle Zeichen der Spieltexte sind enthalten. Am Ende sichtbar: `public/schrift/k3c.ttf` mit Lizenzdatei, Zeichen-Test grün mit `StrictFont`, `task native:game:e2e` mit zwei gleichen Läufen.

## Ergebnis

2026-11-12 Review (Claude, `k3c-reviewer`): Kriterien laut Ergebnis der Sessions, Befunde behoben oder weitergegeben (siehe Review-Session). Version: keine vorgeschlagen.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `public/schrift/k3c.ttf` mit Lizenzdatei, Wahl und Suche in den Notizen (`B-815/AC-01`). |
| AC-02 | – | Alle Zeichen der Spieltexte in der Schrift, `StrictFont` an (`B-815/AC-02`). |
| AC-03 | – | Ende-zu-Ende-Lauf: zwei Läufe gleich, Bilder abgelegt (`B-815/AC-03`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-815 | erledigt | AC-01 –, AC-02 –, AC-03 – | Das native Spiel zeichnet mit einer freien OFL-Schrift statt der Debug-Schrift |
