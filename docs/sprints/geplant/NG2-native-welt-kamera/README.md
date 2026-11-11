# NG2 · CLI · Natives Spiel: Welt, Kamera, Schichten, Fluss und Licht

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-795
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Nach NG1 zeichnet `native/k3c-game/world` nur Platzhalter. Vorbild sind `src/scenes/render/` und `src/scenes/layout.ts` sowie Fluss und Schichten aus NV3 (`native/k3c-ebiten/river.go`, `level.go`). Einzelheiten: B-795.

## Ziel

Jede Zelle zeigt Hintergrund, Boden, Fluss und Licht und führt die Kamera wie das heutige Spiel. Am Ende sichtbar: Monarch reitet, `fern` steht, `mitte` und `nah` ziehen vorbei, der Fluss fließt im unteren Drittel, Nacht wird dunkel.

## Beteiligte und Zielgruppen

Familie, Claude (Umsetzung), 🧑 Ralf (Sichtprüfung in NG9).

## Anforderungen

`B-795 › Anforderungen`.

## Nicht-Ziele

`B-795 › Nicht-Ziele`; Vertrag `view` ändern.

## Regeln und Einschränkungen

Entscheidung 005 und 008. Vertrag `view` aus NG1 ist fest; fehlt etwas, Ticket.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/world/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe
- **Sperre:** –

## Beispiele

`B-795 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-795 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Kamera und Culling je Zelle (`B-795/AC-01`).
- **AC-02** Schichten und Fluss im unteren Drittel (`B-795/AC-02`).
- **AC-03** Licht und Stufenübergang (`B-795/AC-03`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG2.1 | `NG2.1-kamera-culling.md` | Umsetzung | autonom | offen |
| NG2.2 | `NG2.2-schichten-fluss.md` | Umsetzung | autonom | offen |
| NG2.3 | `NG2.3-licht-stufen.md` | Umsetzung | autonom | offen |
| NG2.4 | `NG2.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
