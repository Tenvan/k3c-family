# RV1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Asset-Review mit Git-Diff, Pixel-Overlay und Teil-Markierung
- **Projekt:** PAP

## Ziel

Alle Asset-RPC-Methoden liefern echte Daten, und die Startseite von Assets zeigt jede Asset-Änderung gegen eine wählbare Git-Basis mit Vorher/Nachher, Pixel-Diff-Overlay, Teil-Markierung, synchronen Animationen und Prompt-Buttons. Am Ende sichtbar: Nach einer Änderung am Umhang-Material zeigt die Review-Seite den Monarchen mit umrandetem Umhang.

## Ergebnis

- 2026-11-06, autonom (Claude): AC-01 bis AC-07 mit Tests belegt (Ergebnisse RV1.1–RV1.4, Nachweis RV1.5); `task check`, `task check:go`, `task lint` grün, `check:race` lokal ohne C-Compiler übersprungen. - Behoben: 3 Sonnet-Reviews (RV1.1, RV1.2) und Gesamt-Review Opus (4 Important: Basis-Commit, Neu laden, Frame-Grenze, Diff-Cache). - Neue Tickets: B-703; Defaults D-RV1-1 bis D-RV1-9 in B-701. - Offen: manuelle Abnahme durch 🧑 Ralf (Monarch mit umrandetem Umhang), sobald RD1 und eine Monarch-Spec im Checkout liegen; bis dahin bleibt der Sprint aktiv.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `AssetsReview` liefert für das Test-Repo die Fälle `neu`, `geaendert`, `geloescht`, `ungue … |
| AC-02 | geprüft | Basis `HEAD`, `HEAD~1`, Commit und `origin/develop` wählbar; ungültige Basis → Fehlertext … |
| AC-03 | geprüft | Seite zeigt Vorher/Nachher, Überblendregler, Pixel-Diff-Overlay mit Boxen und markiert geä … |
| AC-04 | geprüft | Animationen laufen alt und neu synchron mit Frame-Schieber. |
| AC-05 | geprüft | Ungültige Spec als roter Eintrag mit Fehlertext, die übrigen werden angezeigt. |
| AC-06 | geprüft | Prompt-Buttons am Review-Eintrag (Vorlagen `ort: review`) kopieren den gefüllten Prompt (0 … |
| AC-07 | geprüft | `AssetsList`, `AssetsGet`, `AssetsStyle`, `AssetsPrompts` liefern echte Daten aus dem Chec … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-699 | eingeplant | – | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review … |
