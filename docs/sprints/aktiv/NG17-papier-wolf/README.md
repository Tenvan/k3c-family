# NG17 · PAP · Papier: Reittier Wolf und Pfoten der Gegner-Wölfe

- **Status:** aktiv
- **Projekt:** NAT
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-804
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

`findeReittier` (`tools/paper/rig/reiten.ts`) durchsucht die Nachbar-Gruppen alphabetisch und findet für `"reittier": "wolf"` den Gegner-Wolf statt des Reittiers (belegt in NG3.1). `gegner/wolf` und `gegner/alphawolf` haben kein `proportionen.fuss` und fallen auf `huf` zurück. Einzelheiten: B-804.

## Ziel

Der Monarch reitet den Reittier-Wolf, Gegner-Wölfe haben Pfoten. Am Ende sichtbar: Tests in `tools/paper` grün, Grafik-Golden nach Regel aktualisiert.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Review); natives Spiel und Browser-Client (laden die Rigs); 🧑 Ralf (Sichtprüfung im Bereich Assets der Workbench, nicht blockierend).

## Anforderungen

`B-804 › Anforderungen`.

## Nicht-Ziele

`B-804 › Nicht-Ziele`; Sim- und Balance-Läufe.

## Regeln und Einschränkungen

Entscheidung 005: Specs nur über `assets_*`. Grafik-Golden nach `docs/arbeitsweise.md` › Golden aktualisieren (Begründung im Commit); `gen:paper` läuft hier, weil die Session das Feature entwickelt (B-742).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `tools/paper/rig/**`, `tools/paper/golden/**`, `public/grafik/gegner/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** golden

## Beispiele

`B-804 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-804 › Ausnahme- und Fehlerfälle` (kein Treffer in `reittiere` → bisherige Suche; nirgends → bisherige Fehlermeldung).

## Akzeptanzkriterien

- **AC-01** Test in `tools/paper`: Monarch-Animationen `*-wolf` nutzen das Reittier (`B-804/AC-01`).
- **AC-02** Gegner-Wölfe mit `proportionen.fuss: pfote`, Grafik-Golden nach Regel aktualisiert (`B-804/AC-02`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG17.1 | `NG17.1-reittier-wolf-pfoten.md` | Umsetzung | autonom | fertig |
| NG17.2 | `NG17.2-review.md` | Review | autonom | fertig |
| NG17.3 | `NG17.3-gegner-wolf-pfote.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
