# TR4 · PLAT, SRV · Bot-Eingabe ohne verlorene Drücke, Beobachter ohne Platz im Raum

- **Status:** aktiv
- **Projekt:** TST
- **Domäne:** PLAT, SRV
- **Reife:** bereit
- **Tickets:** B-354, B-352
- **Start-Commit:** 819c53d3
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25, Revision 1 (Ziel von Ralf: Specs genehmigt)

## Ausgangslage

Die Bot-Eingabe (`src/input/botInput.ts`, TR2) hält je Slot nur das letzte Feed-Kommando und liest es einmal je Frame: kurze Drücke gehen verloren, ein stummer Feed lässt den Monarchen weiterlaufen (B-354). `sim_test` mit Clients belegt einen Platz im Raum, das Protokoll kennt kein Gerät ohne Platz (B-352).

## Ziel

Testläufe mit Bots verlieren keine Eingaben und lassen sich als Beobachter an echte Clients hängen. Am Ende sichtbar: Test der Bot-Eingabe grün, `sim_test` mit Clients hängt sich ohne Platz an.

## Beteiligte und Zielgruppen

Entwickler und Agents, die Testläufe über `sim_test` starten; Agent in PLAT und SRV.

## Anforderungen

B-354, B-352 › Anforderungen (Specs noch Entwurf).

## Nicht-Ziele

Bot-Profile und Balancing (BAL ruht); Lasttest-Änderungen (LT1 erledigt).

## Regeln und Einschränkungen

Eine Protokoll-Änderung bekommt eine eigene Session, die nur Protokoll und beide Enden anpasst (TR4.2, `docs/protocol.md`, `testdata/protocol/`). Testläufe nur über `sim_test`. Der Bot-Feed nimmt nur Loopback an.

## Beispiele

`attack` kommt und geht zwischen zwei Frames → das Spiel sieht einen Druck. Feed verstummt → Monarch steht nach einer Frist still.

## Ausnahme- und Fehlerfälle

Feed fehlt ganz → Log höchstens einmal je Sekunde (🤒), kein Absturz. Beobachter ohne Platz schickt Eingaben → abgelehnt (🚫).

## Akzeptanzkriterien

- **AC-01** Die Bot-Eingabe liefert auch Drücke, die zwischen zwei Frames kommen und gehen, und hält bei stummem Feed an (B-354).
- **AC-02** Ein Beobachter-Gerät kann ohne Platz angehängt werden; `sim_test` mit Clients erlaubt `players` bis 4 (B-352).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| TR4.1 | `TR4.1-bot-feed-druecke.md` | Umsetzung | autonom | fertig |
| TR4.2 | `TR4.2-beobachter-ohne-platz.md` | Umsetzung | autonom | fertig |
| TR4.3 | `TR4.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
