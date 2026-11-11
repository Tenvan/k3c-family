# NG5 · PLAT · Natives Spiel: Eingabe mit Gamepads und Tastatur

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** PLAT
- **Reife:** bereit
- **Tickets:** B-798
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Nach NG1 liefert `native/k3c-game/input` nur einen Platzhalter. Vorbild: `src/input/`, `docs/rules/bedienung.md`, `src/scenes/keyTable.ts`, Prototyp `native/k3c-ebiten/players.go`. Einzelheiten: B-798.

## Ziel

1–2 Gamepads und die Tastatur steuern Spieler und Menüs mit der heutigen Belegung. Am Ende sichtbar: zwei Pads steuern je einen Monarchen, ein Pad kann während des Spiels kommen und gehen.

## Beteiligte und Zielgruppen

Familie, Claude (Umsetzung).

## Anforderungen

`B-798 › Anforderungen`.

## Nicht-Ziele

`B-798 › Nicht-Ziele`; Vertrag `view` ändern.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md` (B zurück, View + Menu reserviert, nur Xbox-Belegung). Vertrag `view` fest. Domäne PLAT (Eingabe), damit der Sprint neben den CLI-Sprints läuft.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/input/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe
- **Sperre:** –

## Beispiele

`B-798 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-798 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Belegung nach `docs/rules/bedienung.md` (`B-798/AC-01`).
- **AC-02** Zuordnung bei Pad kommt, geht, zwei Pads, kein Pad (`B-798/AC-02`).
- **AC-03** B zurück in Menüs, im Spiel unbelegt; View + Menu zurück ins Menü (`B-798/AC-03`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG5.1 | `NG5.1-belegung.md` | Umsetzung | autonom | offen |
| NG5.2 | `NG5.2-gamepads-hotplug.md` | Umsetzung | autonom | offen |
| NG5.3 | `NG5.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
