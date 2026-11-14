# BD5 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Spielmenü zeigt die Welt, legt Spielstände an und löscht sie
- **Projekt:** BED

## Ziel

Spielende sehen im Spielmenü, in welcher Welt sie spielen, und verwalten ihre Spielstände dort selbst. Am Ende sichtbar: Weltname im Menü, neuer Spielstand und Löschen mit Rückfrage, am PC von 🧑 abgenommen.

## Ergebnis

2026-10-22 (BD5.3, autonom): AC-01 (BD5.1, Go-Tests) und AC-02 (BD5.2, Vitest + Browser-Pane) erfüllt. Prüfungen: check, check:go grün, check:race ohne C-Compiler übersprungen. Reviewer: keine schweren Befunde, Rotations-Hinweis in `docs/protocol.md` ergänzt. 2026-10-24 (BD5.4, Ralf am PC): AC-03 erfüllt. Abweichungen als Tickets: B-604 (Welt größer im Pausenmenü, Welt bei jedem Spielstand), B-581 (offenen Raum schließen). Entscheidungen B-533, B-534 gelten (von Ralf nicht beanstandet).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Der Server bietet Anlegen und Löschen von Spielständen mit Welt an, Löschen eines geladene … |
| AC-02 | geprüft | Das Spielmenü zeigt die Welt und legt Spielstände an und löscht sie mit Rückfrage (Vitest … |
| AC-03 | erfüllt | 🧑 hat das Spielmenü am PC abgenommen (B-506). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-506 | verworfen | AC-01 geprüft, AC-02 geprüft, AC-03 erfüllt | Spielmenü zeigt keine Welt und erlaubt kein Anlegen oder Löschen von Spielstände … |
