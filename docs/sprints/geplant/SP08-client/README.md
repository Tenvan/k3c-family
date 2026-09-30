# SP08 · CLI · Browser als reiner Client

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-016, B-018, B-037, B-039
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Browser rechnet die Simulation selbst (`src/world/`) oder spricht Protokoll v1 mit dem Node-Server.

## Ziel

Der Browser rechnet nichts mehr, er schickt Eingaben und zeichnet Snapshots, lokal wie online. Am Ende sichtbar: 2 Controller an der Xbox + 1 Handy im selben Raum.

## Beteiligte und Zielgruppen

Spieler an der Xbox (2 Controller) und am Handy; 🧑 prüft am TV.

## Anforderungen

B-016, B-018, B-037 (einfache Raumwahl) und B-039 › Anforderungen. Sprint-eigen: Couch-Spiel läuft ebenfalls über den Server.

## Nicht-Ziele

Löschen der TS-Simulation (SP09).

## Regeln und Einschränkungen

`src/scenes` rechnet nichts, es zeichnet Snapshots. Seiten-Regeln aus `CLAUDE.md`, B nicht belegen. Prüfungen am TV nur mit Freigabe durch 🧑.

## Beispiele

2 Controller an der Xbox und 1 Handy im selben Raum → drei Monarchen, jeder eigen gesteuert, flüssige Bewegung.

## Ausnahme- und Fehlerfälle

Verbindung weg → Hinweis und Wiederverbinden (B-030) statt Standbild.

## Akzeptanzkriterien

- **AC-01** Der Browser schickt nur Eingaben und zeichnet Snapshots nach Protokoll v2 mit Interpolation (B-039/AC-01).
- **AC-02** Mehrere lokale Spieler pro Gerät nach dem Layout aus B-016 (B-016/AC-02).
- **AC-03** Raum erstellen und beitreten funktioniert (B-037/AC-01, B-037/AC-02).
- **AC-04** `worldRenderer.ts` und `GameScene.ts` haben je ≤ 300 Zeilen (B-018/AC-01, B-018/AC-02).
- **AC-05** Am TV: 2 Controller an der Xbox + 1 Handy spielen im selben Raum (Beobachtung durch 🧑).

## Offene Fragen

Layout für mehr als 2 lokale Spieler (B-016, 🧑).

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SP08.1 Client spricht Protokoll v2, zeichnet Snapshots mit Interpolation; `worldRenderer.ts`/`GameScene.ts` dabei unter 300 Zeilen (AC-01, AC-04).
- SP08.2 Mehrere lokale Spieler pro Gerät (Layout nach B-016), Couch-Spiel ebenfalls über den Server (AC-02).
- SP08.3 Einfache Raumwahl (erstellen, beitreten) (AC-03).
- SP08.4 🔍 Review, 🧑 prüft am TV (AC-05, alle).

## Abnahme

–
