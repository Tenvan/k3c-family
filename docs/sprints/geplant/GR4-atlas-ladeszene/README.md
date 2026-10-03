# GR4 · INF · Atlas und Lade-Szene

- **Status:** geplant
- **Domäne:** INF
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-163, B-029
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Aufgaben nur über `task`; `src/scenes` rechnet nichts; deterministische Atlas-Erzeugung; keine schwere Abhängigkeit ohne Begründung. Voraussetzung: GR1, GR3, X1 für die Messung.

## Beispiele

Start auf der Xbox → Balken läuft, danach Menü; `task atlas` zweimal → byte-gleiche Dateien.

## Ausnahme- und Fehlerfälle

Asset lädt nicht → Meldung statt ewig laufendem Balken (B-029). Quell-Bild fehlt → `task atlas` bricht mit Dateinamen ab.

## Akzeptanzkriterien

- **AC-01** `task atlas` erzeugt Atlas-Bild und -Beschreibung, zweimaliger Lauf liefert byte-gleiche Dateien (B-163/AC-01).
- **AC-02** Das Spiel lädt Figuren und eingebaute Umgebungs-Grafiken aus Atlanten, die Zahl der Requests ist vorher und nachher dokumentiert (B-163/AC-02).
- **AC-03** Beim Start zeigt eine Lade-Szene den Fortschritt, bis alle Assets geladen sind (B-029/AC-01).
- **AC-04** Das Kaltstart-Budget ist festgelegt und am TV gemessen (B-163/AC-03).
- **AC-05** `task check` und `task build` sind grün (B-163/AC-04).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GR4.1 `task atlas` mit deterministischer Ausgabe und Einbindung in `task build` (AC-01).
- GR4.2 Spiel lädt aus Atlanten, Lade-Szene mit Fortschritt und Fehlermeldung (AC-02, AC-03).
- GR4.3 🧑 Messung auf der Xbox, Budget festlegen (AC-04).
- GR4.4 Review (AC-05).

## Abnahme

–
