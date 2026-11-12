# NG18 · PAP · Natives Spiel: freie Schrift statt Debug-Schrift

- **Status:** aktiv
- **Projekt:** NAT
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-815
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Das native Spiel hat keine Schriftdatei und zeichnet mit der Debug-Schrift von Ebitengine (ASCII/Latin-1, feste Größe); „–“ und „→“ fehlen. `native/k3c-game/art` erwartet `public/schrift/k3c.ttf` (`art.DefaultFontPath`), `StrictFont` ist vorbereitet. Einzelheiten: B-815.

## Ziel

Das native Spiel zeichnet mit einer freien Schrift aus Repo oder Go-Modul-Cache, alle Zeichen der Spieltexte sind enthalten. Am Ende sichtbar: `public/schrift/k3c.ttf` mit Lizenzdatei, Zeichen-Test grün mit `StrictFont`, `task native:game:e2e` mit zwei gleichen Läufen.

## Beteiligte und Zielgruppen

Spieler; Claude (Umsetzung, Review); 🧑 Ralf (Nachmeldung der Abweichung von „OFL“, Sichtprüfung bei der nächsten PC-Abnahme, nicht blockierend).

## Anforderungen

`B-815 › Anforderungen` (Revision 2: erst OFL in `node_modules` suchen, sonst Go-Schrift BSD-3, Ablage als Datei).

## Nicht-Ziele

`B-815 › Nicht-Ziele`; Lizenz-Seite im Menü (B-808, NG13), Download aus dem Netz.

## Regeln und Einschränkungen

Entscheidungen 005 und 008; keine Änderung an `go.mod` oder `package.json`. Der Zeichen-Test liegt in `art` (externes Testpaket) oder `e2e`, nicht in `game` (Bereich von NG14 in derselben Welle). NG13 setzt auf den Textmaßen dieser Schrift auf.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `public/schrift/**`, `native/k3c-game/art/**`, `native/k3c-game/e2e/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

`B-815 › Beispiele` („Stufe 1 → Stufe 2“ mit Pfeil).

## Ausnahme- und Fehlerfälle

`B-815 › Ausnahme- und Fehlerfälle`. Ein Zeichen fehlt in der Go-Schrift → Test rot, Text oder Schrift anpassen und im Ergebnis nennen (kein stilles Kästchen).

## Akzeptanzkriterien

- **AC-01** `public/schrift/k3c.ttf` mit Lizenzdatei, Wahl und Suche in den Notizen (`B-815/AC-01`).
- **AC-02** Alle Zeichen der Spieltexte in der Schrift, `StrictFont` an (`B-815/AC-02`).
- **AC-03** Ende-zu-Ende-Lauf: zwei Läufe gleich, Bilder abgelegt (`B-815/AC-03`).

## Offene Fragen

keine (entschieden autonom, Claude, 2026-11-12: Go-Schrift BSD-3 als Default, Abweichung von „OFL“, siehe `B-815 › Offene Fragen`).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG18.1 | `NG18.1-schrift.md` | Umsetzung | autonom | offen |
| NG18.2 | `NG18.2-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
