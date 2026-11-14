# MG10 · PAP · Papierschnitt: Bosse neu mit eigener Silhouette

- **Status:** erledigt
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-720, B-711
- **Start-Commit:** 879fac30d
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

Bosse nebeneinander als schwarze Flächen → jeder an seiner Silhouette erkennbar (Buckel, Felsblock, Kristallzacken, Robe mit Schwanz, Riesenkopf mit Ohr).

## Ausnahme- und Fehlerfälle

Angriff ragt über den Rahmen → Rahmen breiter oder Keyframes enger, Oberkante −175 bleibt (Spielgröße).

## Akzeptanzkriterien

- **AC-01** B-720/AC-01 bis AC-03 (Sichtprüfung der Silhouetten durch 🧑).
- **AC-02** B-711/AC-01.
- **AC-03** Review ohne offenen schweren Befund.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MG10.5 | `MG10.5-reiter-variante.md` | Umsetzung | autonom | fertig |
| MG10.1 | `MG10.1-silhouetten.md` | Umsetzung | autonom | fertig |
| MG10.2 | `MG10.2-bosse-a.md` | Umsetzung | autonom | fertig |
| MG10.3 | `MG10.3-bosse-b.md` | Umsetzung | autonom | fertig |
| MG10.4 | `MG10.4-review.md` | Umsetzung | autonom | fertig |

## Abnahme

- 2026-11-14, Review MG10.4 (k3c-reviewer, Claude): AC-01 umgesetzt, Sichtprüfung durch 🧑 offen (B-720 bleibt offen); AC-02 erfüllt (MG10.5, B-711 erledigt); AC-03 erfüllt.
- Behoben: `ersatz.test.ts` auf die neuen Boss-Teile; Ergebnis MG10.5 korrigiert.
- Neue Tickets: B-885 (nativer Rig-Export der Bosse), B-886 (Abnahme: Sichtprüfung, B-711 bestätigen, `gen:paper`, `check:race`).
- `task check` und `task check:go` grün.
