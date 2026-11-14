# LP1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Landingpage für Spieler, Entwicklerseite für Werkzeuge
- **Projekt:** BED

## Ziel

Die Familie sieht auf der Landingpage nur Spielen und Lizenzen, alle Werkzeuge liegen gegliedert auf einer eigenen Entwicklerseite. Am Ende sichtbar: Landingpage mit Weiterspielen, Neues Spiel, Online spielen, Lizenzen und einer kleinen Kachel „Entwicklung“; diese öffnet `dev.html` mit den Abschnitten Entwicklung, Performance und Balancing.

## Ergebnis

2026-10-07, Review LP1.3 (Agent Claude Opus 5.5): AC-01, AC-03 (LP1.1: `pages.test.ts`, `projectRules.test.ts`, `task check` grün), AC-02 (LP1.1 `devTiles.test.ts`, LP1.2 alle neun Kacheln im Browser-Pane), AC-05 (LP1.2: B-292/AC-01–03, Log `🏰 Raum erstellt`, kein `save_exists`) mit Nachweis; AC-04 angenommen, Validierung offen (LP1.4, 🧑 am PC). Behoben (schwer, hätte LP1.4 verhindert): Entwicklerseite war mit der Tastatur nur per Tab bedienbar; Pfeiltasten wählen jetzt wie auf der Landingpage. Grenzfall `projectRules.test.ts` geprüft: nur die Seiten-Prüfung geändert. B-292 archiviert; neues T …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Landingpage nur mit den Spieler-Kacheln und genau einer kleinen Kachel zur Entwicklerseite … |
| AC-02 | geprüft | Entwicklerseite listet alle bisherigen Werkzeug-Kacheln plus Dungeon Master, gegliedert na … |
| AC-03 | geprüft | `task check` grün, `tests/projectRules.test.ts` deckt die neue Seite ab (B-335/AC-03). |
| AC-04 | – | 🧑 hat Landingpage und Entwicklerseite am PC mit Tastatur abgenommen (B-335/AC-04). |
| AC-05 | – | „Neues Spiel“ startet auch bei vorhandenem Spielstand `familie` (B-292/AC-01, B-292/AC-02, … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-335 | verworfen | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 – | Die Landingpage zeigt nur Spieler-Kacheln, Entwicklungs-, Performance- und Balan … |
| B-292 | erledigt | AC-05 – | Die Kachel „Neues Spiel“ startet auch bei vorhandenem Spielstand familie |
