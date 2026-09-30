# B-055 · Das Komplexitäts-Budget gilt auch für server/*.mjs

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`.oxlintrc.json` (SP01.1) hat `server` in `ignorePatterns`, abweichend von SP01.1 › Schritt 3 und 4 (feste
Ignore-Liste, Ausnahme für jede Datei mit Verstoß). Begründung im Ergebnis von SP01.1: Ziel der Session ist
TypeScript, der Node-Server entfällt mit SP09. Das Budget in `docs/arbeitsweise.md` gilt aber für „Code“, und bis SP09
darf `server/*.mjs` noch geändert werden (Fehlerbehebungen, SRV). Heute prüft dort nichts das Budget oder die
`correctness`-Regeln. Gemessen im Review SP01.4 (Oxlint 1.86.0): Komplexität `server/server.mjs` 16, `server/saves.mjs`
17, sonst keine Verstöße. Die Korrektur im Review wurde nicht freigegeben, deshalb dieses Ticket.

## Ziel

Das Komplexitäts-Budget gilt auch für `server/*.mjs`, bis der Node-Server entfällt. Nutzen: Bis SP09 kann dort keine
Funktion unbemerkt über die harten Grenzen wachsen.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die bis SP09 am Node-Server arbeiten; Review-Session; 🧑 (entscheidet).

## Anforderungen

- `npm run lint` prüft `server/*.mjs` mit den harten Grenzen.
- Die zwei Bestandsverstöße stehen mit gemessenem Wert als Ratsche in `.oxlintrc.json`.

## Nicht-Ziele

`server/*.mjs` umbauen; Ziel-Ratsche im Regel-Test für `server/` (SP01.3 › Nicht-Ziele).

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget.
Mit SP09 entfallen die Ausnahmen zusammen mit `server/`.

## Beispiele

Neue Funktion mit Komplexität 16 in `server/reports.mjs` → `npm run lint` scheitert.

## Ausnahme- und Fehlerfälle

Oxlint meldet in `server/` zusätzliche `correctness`-Warnungen → in B-051 aufnehmen, nicht hier beheben.

## Akzeptanzkriterien

- **AC-01** `server` steht nicht mehr in `ignorePatterns`; `.oxlintrc.json` hat Ausnahmen `server/server.mjs` 16 und `server/saves.mjs` 17.
- **AC-02** Eine Probe-Funktion mit Komplexität 16 in `server/reports.mjs` lässt `npm run lint` scheitern (zurücknehmen).

## Offene Fragen

Lohnt sich das bis SP09, oder bleibt `server/` bewusst ausgenommen (dann als Regel in `docs/arbeitsweise.md` nennen)? (🧑)

## Notizen

Gefunden im Review SP01.4.
