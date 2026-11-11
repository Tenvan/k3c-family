# NG4 · CLI · Natives Spiel: Gebäude, Baustellen, Natur und Objekte

- **Status:** erledigt
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-797
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Nach NG1 zeichnet `native/k3c-game/props` nur Platzhalter. Vorbild: Gebäude-, Platz-, Natur- und Objekt-Darstellung in `src/scenes/render/`. Einzelheiten: B-797.

## Ziel

Gebäude, Baustellen, Plätze, Natur und Objekte erscheinen nativ wie im Browser. Am Ende sichtbar: Bauen am Platz mit Preis-Anzeige, Gerüst und fertigem Gebäude; Bäume, Adern und Münzen.

## Beteiligte und Zielgruppen

Familie, Claude (Umsetzung), 🧑 Ralf (Sichtprüfung in NG9).

## Anforderungen

`B-797 › Anforderungen`.

## Nicht-Ziele

`B-797 › Nicht-Ziele`; Vertrag `view` ändern.

## Regeln und Einschränkungen

Entscheidung 005. Vertrag `view` fest.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/props/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe
- **Sperre:** –

## Beispiele

`B-797 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-797 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Gebäude mit Stufe, Schaden und Ruine (`B-797/AC-01`).
- **AC-02** Baustellen, Gerüste, Schilder, Preis-Anzeigen (`B-797/AC-02`).
- **AC-03** Natur und Objekte (`B-797/AC-03`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG4.1 | `NG4.1-gebaeude.md` | Umsetzung | autonom | fertig |
| NG4.2 | `NG4.2-baustellen-schilder.md` | Umsetzung | autonom | fertig |
| NG4.3 | `NG4.3-natur-objekte.md` | Umsetzung | autonom | fertig |
| NG4.4 | `NG4.4-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-11 Review (Claude, `k3c-reviewer`): Kriterien laut Ergebnis der Sessions. Behobene Befunde: siehe Review-Session. Offen: Schriftdatei fehlt (Debug-Schrift); am echten Gerät ungeprüft (Fenster, Gamepads). Kein Abnahme-Ticket. Version: keine vorgeschlagen (noch kein spielbarer Stand).
