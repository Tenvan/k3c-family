# NV3 · CLI · Prototyp-Ausbau: Fluss, fester Hintergrund, alle Figuren als Rig

- **Status:** aktiv
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-789
- **Start-Commit:** 005c5c9b
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (B-789 Revision 2)

## Ausgangslage

Prototyp 2 (`native/k3c-ebiten`) zeigt ein Wald-Level aus Sheets ohne Fluss, alle Schichten scrollen. Prototyp 3 (`native/k3c-rig`) kann Monarch und Pferd als Rig. Einzelheiten: B-789.

## Ziel

Der Prototyp zeigt das gemeinte Bild des Spiels: Fluss in der unteren Bildhälfte, fester Fern-Hintergrund, Figuren als Rig. Am Ende sichtbar: `task native:ebiten` zeigt im Split-Screen den fließenden Fluss, die stehende Schicht `fern` und Rig-Figuren; ein Messbericht liegt unter `reports/`.

## Beteiligte und Zielgruppen

🧑 Ralf (Sichtprüfung am PC, Entscheidung), Claude (Bau, Messung).

## Anforderungen

`B-789 › Anforderungen`.

## Nicht-Ziele

`B-789 › Nicht-Ziele`; Shader (B-790); Messung am Mini-PC (B-786).

## Regeln und Einschränkungen

Entscheidung 005 und 008, Rig-Format `native/k3c-rig/RIG.md`. Keine Simulations- und Balancing-Läufe (Sperre 2026-11-11); der Messlauf des Prototyps ist eine Leistungsmessung der Darstellung und läuft je einmal.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-ebiten/**`, `native/k3c-rig/**`, `native/Taskfile.yml`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

`B-789 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-789 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Fluss in der unteren Hälfte jeder Sicht, drei Bahnen als fließende Kacheln (`B-789/AC-01`).
- **AC-02** Schicht `fern` steht, `mitte` und `nah` scrollen (`B-789/AC-02`).
- **AC-03** Figuren mit Teile-Baum als Rig, Posen gegen Sollwerte geprüft (`B-789/AC-03`).
- **AC-04** Messbericht Entwicklungs-PC, Rig gegen Sheet (`B-789/AC-04`).
- **AC-05** Sichtprüfung und Entscheidung 🧑 (`B-789/AC-05`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NV3.1 | `NV3.1-fluss-fester-hintergrund.md` | Umsetzung | autonom | fertig |
| NV3.2 | `NV3.2-rig-export-alle-figuren.md` | Umsetzung | autonom | fertig |
| NV3.3 | `NV3.3-rigs-im-level.md` | Umsetzung | autonom | in Arbeit |
| NV3.4 | `NV3.4-messung-review.md` | Review | autonom | offen |
| NV3.5 | `NV3.5-sichtpruefung-pc.md` | Review | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
