# SO4 · CLI · Musik je Zustand

- **Status:** geplant
- **Projekt:** –
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-168
- **Start-Commit:** –
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, Revision 1; mit Änderungen aus der Spec-Prüfung

## Ausgangslage

Audio-Kern (SO1), Effekte (SO2) und Hörprobenseite (SO3) stehen; Musik fehlt. Details in B-168.

## Ziel

Die Musik wechselt je Spielzustand mit Crossfade. Am Ende sichtbar: Die 8 Zustände Tag, Abend (= Dämmerung, Q65), Nacht, Kampf, Tiefe/Höhle, Boss, Lobby und Niederlage/Sieg haben je 1–2 eigene Stücke, Warnungen senken die Musik.

## Beteiligte und Zielgruppen

🧑 wählt Stücke mit `soundtest.html` (Q16); Agent baut ein; Spieler am TV.

## Anforderungen

B-168 › Anforderungen.

## Nicht-Ziele

Eigene Kompositionen, Effekte (SO2), dynamische Layer.

## Regeln und Einschränkungen

`src/scenes` rechnet nichts; CC0 oder CC-BY mit Credits (Credits-Seite B-165); eine Musik je Gerät; B nicht belegen.

## Beispiele

Nacht beginnt → Crossfade zu „Nacht“.

## Ausnahme- und Fehlerfälle

Stück fehlt → vorheriger Zustand oder Stille, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Der Zustands-Automat bildet jeden der 8 Zustände auf 1–2 Stücke ab (Q16) (B-168/AC-01).
- **AC-02** Der Wechsel Tag → Dämmerung → Nacht → Morgengrauen (Q65) ist hörbar ohne Pause und Knacken (B-168/AC-02).
- **AC-03** Ein Warn-Sound senkt die Musik ab und stellt sie wieder her (B-168/AC-03).
- **AC-04** Jede Musik-Datei hat einen Credit-Eintrag, CC-BY-Stücke stehen auf der Credits-Seite (B-168/AC-04).
- **AC-05** Die Lautstärke des Busses „Musik“ ist getrennt von den Effekten einstellbar (B-168/AC-05).
- **AC-06** `task check` ist grün.
- **AC-07** Die Lautheit der Musik ist am TV geprüft (Q16) (B-168/AC-06).

## Offene Fragen

- Welches Stück läuft im Morgengrauen? Q65 führt die Phase ein, Q16 nennt dafür keinen eigenen Zustand (B-168 › Offene Fragen, B-213).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SO4.1 | `SO4.1-workshop-stuecke.md` | Workshop | Mensch | offen |
| SO4.2 | `SO4.2-zustandsautomat-crossfade.md` | Umsetzung | autonom | offen |
| SO4.3 | `SO4.3-ducking-dateien-credits.md` | Umsetzung | autonom | offen |
| SO4.4 | `SO4.4-review.md` | Review | autonom | offen |
| SO4.5 | `SO4.5-hoerprobe-tv.md` | Workshop | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
