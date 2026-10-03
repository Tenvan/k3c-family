# GR4 · INF · Atlas und Lade-Szene

- **Status:** aktiv
- **Domäne:** INF
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-163, B-029
- **Start-Commit:** 605f467
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1, durch 🧑; umfasst B-163, B-029; mit Änderungen aus dem Spec-Review (Voraussetzungen, AC-06 Ladefehler, Texturgröße in GR4.3)

## Ausgangslage

`src/scenes/sprites.ts` lädt jede Figur-Animation als eigene PNG, es gibt weder Atlas noch Lade-Szene noch Messung des Kaltstarts auf der Xbox. Details in B-163 und B-029.

## Ziel

Die Grafiken kommen aus Atlanten, die Lade-Szene zeigt Fortschritt, der Kaltstart hat ein gemessenes Budget. Am Ende sichtbar: `task atlas`, ein Balken beim Start, ein Messwert vom TV.

## Beteiligte und Zielgruppen

Spieler am TV; Entwickler und Agenten; 🧑 misst an der Xbox (nach X1).

## Anforderungen

B-163 › Anforderungen und B-029 › Anforderungen. Hinweis: B-029 ist ein CLI-Ticket, der Sprint ist INF (Zuordnung laut Gesamtplan).

## Nicht-Ziele

Nachladen im Hintergrund, neue Grafiken (GR2), Effekte (GR5).

## Regeln und Einschränkungen

Aufgaben nur über `task`; `src/scenes` rechnet nichts; deterministische Atlas-Erzeugung; keine schwere Abhängigkeit ohne Begründung. Keine harte Voraussetzung: GR4.1 nimmt die Figurenliste aus `data/sprites.json`; ohne GR3 betrifft AC-02 nur die Figuren; X1 nur für die Messung (GR4.3).

## Beispiele

Start auf der Xbox → Balken läuft, danach Menü; `task atlas` zweimal → byte-gleiche Dateien.

## Ausnahme- und Fehlerfälle

Asset lädt nicht → Meldung statt ewig laufendem Balken (B-029). Quell-Bild fehlt → `task atlas` bricht mit Dateinamen ab.

## Akzeptanzkriterien

- **AC-01** `task atlas` erzeugt Atlas-Bild und -Beschreibung, zweimaliger Lauf liefert byte-gleiche Dateien (B-163/AC-01).
- **AC-02** Das Spiel lädt Figuren und eingebaute Umgebungs-Grafiken aus Atlanten, die Zahl der Requests ist vorher und nachher dokumentiert (B-163/AC-02).
- **AC-03** Beim Start zeigt eine Lade-Szene den Fortschritt, bis alle Assets geladen sind (B-029/AC-01).
- **AC-04** Das Kaltstart-Budget ist festgelegt und am TV gemessen, die maximale Texturgröße der Xbox ist abgelesen (B-163/AC-03).
- **AC-05** `task check` und `task build` sind grün (B-163/AC-04).
- **AC-06** Lädt ein Asset nicht, zeigt die Lade-Szene eine Meldung mit Dateinamen statt eines hängenden Balkens (B-029/AC-02).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GR4.1 | `GR4.1-task-atlas.md` | Umsetzung | autonom | fertig |
| GR4.2 | `GR4.2-laden-ladeszene.md` | Umsetzung | autonom | fertig |
| GR4.3 | `GR4.3-messung-xbox.md` | Workshop | Mensch | offen |
| GR4.4 | `GR4.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
