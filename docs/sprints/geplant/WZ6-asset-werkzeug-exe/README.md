# WZ6 · DEV · Asset-Werkzeug als eigene EXE mit Seiten für Grafik und Sound

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

Ein eigenes Asset-Werkzeug (EXE mit festem Pfad, eigener Task) mit je einer Seite für Grafik und Sound; die Workbench ist davon entkoppelt. Am Ende sichtbar: beide Seiten im neuen Fenster, Workbench ohne Assets-Seite, von 🧑 abgenommen.

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
- **AC-03** `B-459/AC-03` Workbench ohne Assets-Seite, Neubau des Asset-Werkzeugs stört sie nicht.
- **AC-04** `B-459/AC-04` Tests im Prüflauf.
- **AC-05** `B-459/AC-05` Domänentabelle und Glossar.
- **AC-06** 🧑 nimmt beide Seiten und die ruhige Workbench ab (Review-Session).

## Offene Fragen

B-459 › Offene Fragen 1–4 (Technik Wails oder Web, MCP-Tools `assets_*`, Name, Zukunft von `assetvorschau.html`/`grafiken.html`/`soundtest.html`). Entscheidet 🧑 im Workshop WZ6.1.

## Sessions

- WZ6.1 (DEV) Workshop: Technik, Name, MCP-Schnitt, zwei Schreiber auf `data/assets.json`, Umgang mit den Dev-Seiten festlegen (AC-01–AC-03).
- WZ6.2 (INF) Domänentabelle um `tools/k3c-assets/` und Glossar-Begriffe ergänzen (AC-05).
- WZ6.3 (DEV) Gerüst `tools/k3c-assets`: EXE, Task, Fenster, gemeinsame Go-Pakete mit k3c-dev (AC-01, AC-04).
- WZ6.4 (DEV) Seite Grafik umziehen (AC-02).
- WZ6.5 (DEV) Seite Sound als eigene Seite umziehen (AC-02).
- WZ6.6 (DEV) Workbench: Assets-Seite entfernen, Verweis auf das Asset-Werkzeug, Aufräumen (AC-03).
- WZ6.7 (DEV) Review und Abnahme durch 🧑 (AC-06, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
