# U5 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Debug-Overlay und Cheat-Dialog bedienbar
- **Projekt:** BED

## Ziel

Overlay, Aktionsliste und Cheat-Dialog sind mit Maus, Tastatur und Controller bedienbar und verdecken das HUD nicht. Am Ende sichtbar: Ö schließt Overlay und Liste, HUD bleibt lesbar, Cheat-Dialog mit Fokus und Controller.

## Ergebnis

Review 2026-10-09 (U5.3, eigener Review-Agent): `task check` grün; ein schwerer Befund behoben: Bei offenem Cheat-Dialog lief das Skill-Menü weiter, Leertaste/A hätte Skills lernen oder zurücksetzen können (`GameScene`: `route` ohne Eingaben, Menüs schließen). - AC-01, AC-03 geprüft (U5.1); AC-02 geprüft für Tests und Browser-Pane (U5.2); AC-05 geprüft. B-192 erledigt. - Verschoben nach U5.4: B-191/AC-02, B-317/AC-03, B-317/AC-04 und AC-04 (Xbox-Prüfung; Befund und Prüfauftrag im Ergebnis von U5.2). - Abnahme U5.4 (🧑, 2026-10-10, PC und Touch in Chrome): AC-02 bestanden (Cheat-Dialog mit Fokus …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Die Dev-Aktionsliste schließt sich mit Ö (B-192/AC-01, B-192/AC-02). |
| AC-02 | geprüft | Der Cheat-Dialog zeigt den Fokus und lässt sich mit Pfeiltasten, Leertaste und Controller … |
| AC-03 | geprüft | Debug-Overlay und Aktionsliste verdecken das HUD nicht (B-191/AC-01, B-191/AC-02). |
| AC-04 | – | Die Ursache, warum der Controller den Cheat-Dialog auf der Xbox nicht bedient, ist mit ein … |
| AC-05 | geprüft | `task check` ist grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-192 | erledigt | AC-01 geprüft | Die Dev-Aktionsliste schließt sich mit Ö |
| B-317 | verworfen | AC-02 geprüft | Der Cheat-Dialog zeigt den Fokus und lässt sich mit Pfeiltasten, Leertaste und C … |
| B-191 | verworfen | AC-03 geprüft | Debug-Overlay und Aktionsliste verdecken das HUD nicht |
