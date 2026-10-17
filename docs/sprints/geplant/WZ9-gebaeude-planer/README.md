# WZ9 · DEV · Gebäude-Planer für die Freischaltung je Hub-Stufe

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-462
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-462 › Ausgangslage: Die Freischaltung je Hub-Stufe steht in `data/hub.json` › `sites`/`islandSites` und wird von Hand gepflegt; GEB9 und B-458 hängen daran. Voraussetzung: Asset-Werkzeug (WZ6), Live-Vorschau (WZ7).

## Ziel

Ein Gebäude-Planer im Asset-Werkzeug zeigt und bearbeitet, welches Gebäude ab welcher Hub-Stufe an welchem Platz verfügbar ist, prüft und schreibt `data/hub.json` und zeigt das Ergebnis in der Live-Vorschau. Am Ende von 🧑 abgenommen.

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

- **AC-01** `B-462/AC-01` Matrix Hub-Stufe × Gebäude mit Platz und Tiefe.
- **AC-02** `B-462/AC-02` Geprüftes Schreiben von `data/hub.json`, stabiles Format.
- **AC-03** `B-462/AC-03` Abweichungen zur Regeltabelle sichtbar.
- **AC-04** `B-462/AC-04` Live-Vorschau zeigt die Änderung.
- **AC-05** `B-462/AC-05` Hinweis auf REG-Änderung und Goldens.
- **AC-06** 🧑 nimmt den Planer ab (Review-Session).

## Offene Fragen

B-462 › Offene Fragen 1–3 (Prüfung über Go-Backend, Schreibrecht auf `data/hub.json`, Regeltabelle aus JSON erzeugen). Reihenfolge zu GEB9/B-458: Der Planer erleichtert B-458, ist aber keine Voraussetzung für GEB9.

## Sessions

- WZ9.1 (DEV) Matrix lesen und anzeigen, Abgleich mit der Regeltabelle (AC-01, AC-03).
- WZ9.2 (DEV) Bearbeiten mit Prüfung über das Go-Backend, Schreiben von `data/hub.json`, REG-Hinweis (AC-02, AC-05).
- WZ9.3 (DEV) Anbindung an die Live-Vorschau (AC-04).
- WZ9.4 (DEV) Review und Abnahme durch 🧑 (AC-06, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
