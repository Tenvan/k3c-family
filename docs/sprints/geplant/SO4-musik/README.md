# SO4 · CLI · Musik je Zustand

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-168
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Audio-Kern (SO1), Effekte (SO2) und Hörprobenseite (SO3) stehen; Musik fehlt. Details in B-168.

## Ziel

Die Musik wechselt je Spielzustand mit Crossfade. Am Ende sichtbar: Tag, Abend, Nacht, Kampf, Tiefe, Boss und Lobby haben eigene Stücke, Warnungen senken die Musik.

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

- **AC-01** Der Zustands-Automat bildet jeden Zustand auf genau ein Stück ab (B-168/AC-01).
- **AC-02** Der Wechsel Tag → Abend → Nacht ist hörbar ohne Pause und Knacken (B-168/AC-02).
- **AC-03** Ein Warn-Sound senkt die Musik ab und stellt sie wieder her (B-168/AC-03).
- **AC-04** Jede Musik-Datei hat einen Credit-Eintrag, CC-BY-Stücke stehen auf der Credits-Seite (B-168/AC-04).
- **AC-05** Die Lautstärke des Busses „Musik“ ist getrennt von den Effekten einstellbar (B-168/AC-05).
- **AC-06** `task check` ist grün.

## Offene Fragen

- Zustände und Stücke: Entscheidet 🧑 (`docs/fragenkatalog.md` Q16); blockiert die Freigabe.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SO4.1 🧑 Workshop (Agent: Mensch): Stücke je Zustand mit `soundtest.html` wählen (Grundlage für AC-01).
- SO4.2 Zustands-Automat, Crossfade, Bus „Musik“ (AC-01, AC-02, AC-05).
- SO4.3 Ducking bei Warnungen, Dateien und Credits (AC-03, AC-04).
- SO4.4 Review (AC-06).

## Abnahme

–
