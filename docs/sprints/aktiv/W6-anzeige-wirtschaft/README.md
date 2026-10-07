# W6 · CLI · Anzeigen für Bau, Lager, Hub und Bürger

- **Status:** aktiv
- **Domäne:** CLI
- **Prio:** mittel
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-117, B-126
- **Start-Commit:** 82297cda
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑, Revision 1

## Ausgangslage

Der Client zeigt weder Wartegrund, Lagerstand und Hub-Stufe noch Berufe, Händler, Truppen-Limit und Heilplatz (B-117, B-126).

## Ziel

Spieler sehen, warum ein Bauplatz wartet, Lagerstand und Hub-Stufe mit Kosten sowie Berufe, Händler, Limit und Heilplatz.

Am Ende sichtbar: HUD und Bauplätze am TV, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Spieler (1 bis 4 am TV); 🧑 nimmt die Anzeige am Gerät ab.

## Anforderungen

B-117 › Anforderungen, B-126 › Anforderungen.

## Nicht-Ziele

Grafik-Anbindung (GR3), Ton, Protokoll (W5).

## Regeln und Einschränkungen

Der Client rechnet nichts und zeichnet nur Server-Zustand (`src/scenes/noSim.test.ts`); Logik als reine Funktionen mit Test. Mindest-Schriftgrößen je Split-Viertel nach den Regeln aus F1 (B-136). Seiten-Regeln aus `CLAUDE.md`; B nicht belegen, View + Menu reserviert. Datei ≤ 400 Zeilen, Funktion ≤ 60. Der Sprint bleibt in der Domäne CLI.

## Beispiele

Bauplatz wartet auf Material → Symbol und Text „Material fehlt“ am Platz.

## Ausnahme- und Fehlerfälle

Händler nicht anwesend → keine Händler-Anzeige; Server-Feld fehlt (älterer Server) → Anzeige bleibt leer, kein Fehler.

## Akzeptanzkriterien

- **AC-01** Reine Funktionen für Wartegrund, Lagerstand-Text, Limit-Text und Berufsanzeige sind getestet (`task test`) (B-117/AC-01, B-126/AC-01).
- **AC-02** HUD und Bauplätze zeigen Wartegrund, Lagerstand, Hub-Stufe und Kosten (B-117/AC-02).
- **AC-03** Berufe, Limit, Händler und Heilplatz werden angezeigt (B-126/AC-02).
- **AC-04** 🧑 hat beide Anzeigen am Gerät abgenommen (B-117/AC-03, B-126/AC-03).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W6.1 | `W6.1-reine-funktionen.md` | Umsetzung | autonom | fertig |
| W6.2 | `W6.2-hud-bauplaetze.md` | Umsetzung | autonom | offen |
| W6.3 | `W6.3-buerger-ui.md` | Umsetzung | autonom | offen |
| W6.4 | `W6.4-abnahme-geraet.md` | Workshop | Mensch | offen |
| W6.5 | `W6.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
