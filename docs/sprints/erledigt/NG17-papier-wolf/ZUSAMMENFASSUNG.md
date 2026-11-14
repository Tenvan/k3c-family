# NG17 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Papier: Reittier Wolf und Pfoten der Gegner-Wölfe
- **Projekt:** NAT

## Ziel

Der Monarch reitet den Reittier-Wolf, Gegner-Wölfe haben Pfoten. Am Ende sichtbar: Tests in `tools/paper` grün, Grafik-Golden nach Regel aktualisiert.

## Ergebnis

2026-11-12: AC-01 in NG17.1, AC-02 in NG17.1 (Alphawolf) und NG17.3 (Gegner-Wolf, nach B-819). Review NG17.2 ohne schweren Befund; Golden-Änderungen gewollt (figuren: Reittier-Suche, gegner: Pfoten). Neues Ticket B-819 (erledigt). Gen lief in den Feature-Sessions; Sim und Balance gesperrt (Beschluss 2026-11-11), kein Abnahme-Ticket nötig.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Test in `tools/paper`: Monarch-Animationen `*-wolf` nutzen das Reittier (`B-804/AC-01`). |
| AC-02 | teilweise | Gegner-Wölfe mit `proportionen.fuss: pfote`, Grafik-Golden nach Regel aktualisiert (`B-804 … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-804 | erledigt | AC-01 –, AC-02 teilweise | tools/paper findet für das Reittier Wolf das Reittier statt des Gegners, Gegner- … |
