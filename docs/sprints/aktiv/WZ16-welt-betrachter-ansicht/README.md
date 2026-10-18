# WZ16 · DEV · Welt-Betrachter: eine gemeinsame Ansicht mit Sprungleiste, Zoom und Gebäude-Filter

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-477
- **Start-Commit:** 64c2da42
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-18, Chat (Ralf: „WZ16 ist freigegeben (Sprint-Spec so)“)

## Ausgangslage

`B-477 › Ausgangslage`: Bei der Abnahme von WZ7 (WZ7.7, 2026-10-18) hat 🧑 den Welt-Betrachter im Asset-Studio nicht abgenommen. Chunk-Vorschau (`tools/k3c-studio/frontend/src/world/ChunkPreview.tsx`) und Live-Vorschau (`src/stage/LivePreview.tsx`, Phaser-Bühne `PreviewStage.ts`) sind zwei getrennte Blöcke in `world/WorldPage.tsx`. Voraussetzung: WZ7 erledigt.

## Ziel

Der Welt-Betrachter zeigt Chunk-Streifen und Live-Vorschau in einer Ansicht, die gemeinsam scrollt und zoomt, mit Sprungleiste und gruppiertem Gebäude-Filter; maximiert sind beide Vorschauen sichtbar und nichts wird abgeschnitten.

Am Ende sichtbar: im Studio-Fenster die überarbeitete Seite Welt-Betrachter; `task check:all` und CI grün; von 🧑 abgenommen (damit gilt auch WZ7/AC-09 als erfüllt).

## Beteiligte und Zielgruppen

`B-477 › Beteiligte und Zielgruppen`; Agenten setzen die Sessions um, ein getrennter Reviewer prüft (WZ16.3), 🧑 nimmt am PC ab (WZ16.4).

## Anforderungen

`B-477 › Anforderungen`. Sprint-eigen: Die Seite wird in WZ16.1 (Ansicht, gemeinsames Scrollen, Sprungleiste, Zoom, Layout) und WZ16.2 (Gebäude-Filter) umgebaut; die Phaser-Bühne bleibt die wiederverwendbare Bühne aus WZ7.3 und bekommt Zoom und Scroll-Position von außen.

## Nicht-Ziele

`B-477 › Nicht-Ziele`: keine Änderung an Go, `src/scenes/`, `src/online/`; Chunk-Planung (WZ8, B-461) und Gebäude-Planer (WZ9, B-462) folgen danach auf dieser Ansicht.

## Regeln und Einschränkungen

`B-477 › Regeln und Einschränkungen`. Nur Domäne DEV (`tools/k3c-studio/frontend/`). Richtwert je Session ≤ ~400 geänderte Code-Zeilen; Fenster- und Browserprüfungen nur mit Freigabe von 🧑 für den Lauf. WZ8.5 und WZ9.2 bauen auf der Ansicht auf: WZ8.5 (DEV) und WZ9.2/9.3 ändern dieselben Dateien in `world/` und `stage/`: Sie starten erst, wenn WZ16 erledigt ist (die SIM- und SRV-Sessions von WZ8 laufen unabhängig weiter).

## Beispiele

`B-477 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-477 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** `B-477/AC-01` gemeinsame Ansicht mit gemeinsamem Scrollen, eine Scrollbar.
- **AC-02** `B-477/AC-02` Sprungleiste (links, Mitte, rechts, Hub, Tor, Portal, Ausgang).
- **AC-03** `B-477/AC-03` Zoom stufenlos und in festen Stufen für beide Vorschauen.
- **AC-04** `B-477/AC-04` maximiert beide Vorschauen sichtbar, keine abgeschnittenen Bedienelemente, keine zweite Scrollbar.
- **AC-05** `B-477/AC-05` Gebäude-Filter nach Art gruppiert.
- **AC-06** `B-477/AC-06` `task check:all` und CI grün.
- **AC-07** 🧑 nimmt den überarbeiteten Welt-Betrachter am PC ab (Abnahme-Session; erfüllt auch WZ7/AC-09).

## Offene Fragen

keine. Freigabe der Sprint-Spec steht aus (die Ticket-Spec B-477 ist am 2026-10-18 freigegeben).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ16.1 | `WZ16.1-gemeinsame-ansicht.md` | Umsetzung | autonom | fertig |
| WZ16.2 | `WZ16.2-gebaeude-filter.md` | Umsetzung | autonom | fertig |
| WZ16.3 | `WZ16.3-review.md` | Review | autonom | offen |
| WZ16.4 | `WZ16.4-abnahme.md` | Review | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
