# NG7 · CLI · Natives Spiel: Menüs, Lobby, Spielstände, Optionen und Pause

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-800
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Nach NG1 liefert `native/k3c-game/menu` nur einen Platzhalter, `local` erfüllt `view.Session`. Vorbild: Lobby-, Lade-, Options- und Pause-Logik in `src/scenes/`. Einzelheiten: B-800.

## Ziel

Das Spiel führt vom Start bis ins Spiel und zurück. Am Ende sichtbar: Hauptmenü → Lobby mit zwei Spielern → Spiel → Pause → Hauptmenü → Weiter.

## Beteiligte und Zielgruppen

Familie, Claude (Umsetzung), 🧑 Ralf (Entscheidung Optionen, Sichtprüfung in NG9).

## Anforderungen

`B-800 › Anforderungen`.

## Nicht-Ziele

`B-800 › Nicht-Ziele`; Vertrag `view` ändern.

## Regeln und Einschränkungen

B zurück in jedem Menü, View + Menu reserviert. Kein Import von `local`, nur `view.Session`. Vertrag `view` fest.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/menu/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe, Entscheidung
- **Sperre:** –

## Beispiele

`B-800 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-800 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Hauptmenü und Menü-Gerüst (`B-800/AC-01`).
- **AC-02** Lobby mit 1–2 Spielern (`B-800/AC-02`).
- **AC-03** Spielstände über `view.Session` (`B-800/AC-03`).
- **AC-04** Optionen, Pause, Spielende (`B-800/AC-04`).

## Offene Fragen

- Optionen nativ (`B-800 › Offene Fragen`). Vorschlag: Lautstärke Musik und Effekte, Vollbild, Sprache.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG7.1 | `NG7.1-menue-geruest.md` | Umsetzung | autonom | offen |
| NG7.2 | `NG7.2-lobby.md` | Umsetzung | autonom | offen |
| NG7.3 | `NG7.3-spielstaende.md` | Umsetzung | autonom | offen |
| NG7.4 | `NG7.4-optionen-pause.md` | Umsetzung | autonom | offen |
| NG7.5 | `NG7.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
