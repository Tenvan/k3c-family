# NG11 · CLI · Natives Spiel: Fluss mit Shader

- **Status:** aktiv
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-790
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Nach NG2 zeichnet `native/k3c-game/world` den Fluss als Kacheln mit sechs Wellen-Reihen. Shader (Kage) gibt es im Projekt noch nicht. Einzelheiten: B-790 (Revision 2: Bau im nativen Spiel statt im Prototyp).

## Ziel

Ein Fluss mit Shader läuft umschaltbar neben der Kachel-Fassung, gemessen und von 🧑 entschieden. Am Ende sichtbar: Taste schaltet zwischen beiden Fassungen, Bildrate und Rechenzeit stehen im Bild und im Bericht.

## Beteiligte und Zielgruppen

🧑 Ralf (Sichtprüfung, Entscheidung), Claude (Bau, Messung).

## Anforderungen

`B-790 › Anforderungen`.

## Nicht-Ziele

`B-790 › Nicht-Ziele`; weitere Shader-Effekte (eigene Tickets nach der Entscheidung).

## Regeln und Einschränkungen

`B-790 › Regeln und Einschränkungen`. Shader-Quelle unter `world/water/`, der Umschalter hängt am Fluss in `world/`; `world/` gehört NG2 und ist dann gemergt, in Welle 3 ändert sonst niemand `world/`. Messlauf je Fassung einmal.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/world/**`
- **Abhängig von:** NG2
- **Braucht 🧑:** Freigabe, Abnahme
- **Sperre:** –

## Beispiele

`B-790 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-790 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Fluss mit Shader, umschaltbar, Screenshot beider Fassungen (`B-790/AC-01`).
- **AC-02** Messbericht beider Fassungen vom Entwicklungs-PC (`B-790/AC-02`).
- **AC-03** Entscheidung 🧑 in den Notizen (`B-790/AC-03`).

## Offene Fragen

- Weitere Effekte und Bau im Spiel statt Prototyp (`B-790 › Offene Fragen`).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG11.1 | `NG11.1-fluss-shader.md` | Umsetzung | autonom | fertig |
| NG11.2 | `NG11.2-messung.md` | Umsetzung | autonom | fertig |
| NG11.3 | `NG11.3-review.md` | Review | autonom | offen |
| NG11.4 | `NG11.4-sichtpruefung.md` | Review | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
