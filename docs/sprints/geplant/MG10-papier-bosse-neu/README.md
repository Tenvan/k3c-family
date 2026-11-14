# MG10 · PAP · Papierschnitt: Bosse neu mit eigener Silhouette

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-720
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

PS1.3: Alle Bosse auf Krieger-Grundform; D-MG5-04.

## Ziel

Bosse neu, jeder mit eigener Silhouette.

## Beteiligte und Zielgruppen

Claude setzt um (Assets über MCP); 🧑 prüft die Silhouetten im Review der Workbench.

## Anforderungen

- **Tickets:** B-720, B-711

## Nicht-Ziele

Kampfwerte.

## Regeln und Einschränkungen

Nach RD2 (Rig-Proportionen, Materialien).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `public/grafik/bosse/**`, `public/grafik/figuren/**`, `public/grafik/reittiere/**`, `tools/paper/**`, `docs/standards/paper-spec.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** B-720/AC-01 bis AC-03 (Sichtprüfung der Silhouetten durch 🧑).
- **AC-02** B-711/AC-01.
- **AC-03** Review ohne offenen schweren Befund.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MG10.5 | `MG10.5-reiter-variante.md` | Umsetzung | autonom | offen |
| MG10.1 | `MG10.1-silhouetten.md` | Umsetzung | autonom | offen |
| MG10.2 | `MG10.2-bosse-a.md` | Umsetzung | autonom | offen |
| MG10.3 | `MG10.3-bosse-b.md` | Umsetzung | autonom | offen |
| MG10.4 | `MG10.4-review.md` | Umsetzung | autonom | offen |

Reife Entwurf:

- MG10.1 Silhouetten-Entwürfe aller Bosse
- MG10.2 Bosse A
- MG10.3 Bosse B
- MG10.4 Review

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
