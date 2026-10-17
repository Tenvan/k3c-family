# WZ6 · DEV · Asset-Studio als eigene EXE mit Seiten für Grafik und Sound

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-459
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-459 › Ausgangslage: Die Assets-Seite steckt in k3c-dev; jede geplante Änderung stört die Workbench (Neubau, MCP-Ausfall). Grafik und Sound teilen sich eine Seite.

## Ziel

Ein eigenes Asset-Studio (EXE mit festem Pfad, eigener Task) mit je einer Seite für Grafik und Sound; die Workbench ist davon entkoppelt. Am Ende sichtbar: beide Seiten im neuen Fenster, Workbench ohne Assets-Seite, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `B-459/AC-01` eigene EXE mit festem Pfad und Task, nur Loopback.
- **AC-02** `B-459/AC-02` Seiten Grafik und Sound mit heutigem Umfang.
- **AC-03** `B-459/AC-03` Workbench ohne Assets-Seite, Neubau des Asset-Studios stört sie nicht.
- **AC-04** `B-459/AC-04` Tests im Prüflauf.
- **AC-05** `B-459/AC-05` Domänentabelle und Glossar.
- **AC-06** `B-459/AC-06` `assetvorschau.html` umgezogen und entfernt; Inhalte von `grafiken.html` und `soundtest.html` zusätzlich im Asset-Studio, beide Seiten bleiben.
- **AC-07** `B-459/AC-07` MCP bleibt in k3c-dev, Neuladen bei Änderung von `data/assets.json`.
- **AC-08** 🧑 nimmt beide Seiten, die umgezogene Vorschau und die ruhige Workbench ab (Abnahme-Session).

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (B-459 › Offene Fragen): Wails-App `tools/k3c-studio`, Name Asset-Studio, `assets_*` bleiben in k3c-dev, `assetvorschau.html` zieht um, `soundtest.html` und `grafiken.html` bleiben für die Xbox, Domäne DEV. Hinweis: Das Kopf-Feld Domäne zeigt nur DEV, bis die Session-Dateien (INF, DEV, PLAT) beim Bereitmachen angelegt sind. Offen: Freigabe der Spec.

## Sessions

- WZ6.1 (INF) Domänentabelle um `tools/k3c-studio/` (DEV) und Glossar um Asset-Studio und Folgebegriffe ergänzen (AC-05).
- WZ6.2 (DEV) Gerüst `tools/k3c-studio`: Wails-App, EXE unter `bin/`, Task, Fenster, gemeinsame Go-Pakete mit k3c-dev, Neuladen bei Änderung von `data/assets.json` (AC-01, AC-04, AC-07).
- WZ6.3 (DEV) Seite Grafik umziehen, dazu Inhalt von `assetvorschau.html` und `grafiken.html` (AC-02, AC-06).
- WZ6.4 (DEV) Seite Sound umziehen, dazu Inhalt von `soundtest.html` (AC-02, AC-06).
- WZ6.5 (DEV) Workbench: Assets-Seite entfernen, Verweis auf das Asset-Studio, Aufräumen (AC-03).
- WZ6.6 (PLAT) `assetvorschau.html` samt `src/tools/assetvorschau*.ts` (mit Tests), Kachel in `src/tools/devTiles.ts`, Texten und Verweisen (`docs/standards/gebaeude-grafik.md`, `tests/projectRules.test.ts` falls betroffen) entfernen; `task check` bleibt grün (AC-06).
- WZ6.7 (PLAT) Review (AC-01–AC-07, alle).
- WZ6.8 (PLAT, Mensch) Abnahme durch 🧑 am PC (AC-08).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
