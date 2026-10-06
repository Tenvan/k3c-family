# GR7 · CLI · Figuren-Lücken, ganzzahlige Skalierung und Schrift

- **Status:** geplant
- **Domäne:** CLI
- **Prio:** mittel
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-193, B-251, B-198, B-197, B-320
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Figuren-Lücken unter `public/sprites/` haben keine Kandidaten (B-193), Figuren flimmern bei krummer Skalierung (B-251), der Platzhaltertext einer ungeladenen Stufe hat keine Schrift-Rolle (B-198), die Mindestgröße der Mitspieler-Zelle ist offen (B-197).

## Ziel

Figuren sehen am TV sauber aus, jede Lücke hat eine Auswahl von 🧑, Schrift folgt dem Katalog. Am Ende sichtbar: Figuren ohne Flimmern, Lücken mit Auswahl, Schrift-Mindestgrößen geklärt.

## Beteiligte und Zielgruppen

🧑 wählt Figuren und entscheidet B-197; Agent baut.

## Anforderungen

B-193 › Anforderungen; B-251 › Anforderungen; B-198 › Anforderungen; B-197 › Anforderungen.

## Nicht-Ziele

Atlas (GR4), neue Animationen.

## Regeln und Einschränkungen

Einschiebbar; CC0/CC-BY mit Credits.

## Beispiele

Layout 4 → Platzhaltertext in Mindestgröße der Rolle `world`.

## Ausnahme- und Fehlerfälle

Keine passende Figur → Lücke bleibt mit Ticket, Platzhalter-Rückfall.

## Akzeptanzkriterien

- **AC-01** Figuren-Lücken unter public/sprites/ haben Kandidaten und eine Auswahl (B-193/AC-01, B-193/AC-02, B-193/AC-03).
- **AC-02** Figuren werden ganzzahlig skaliert und flimmern nicht (B-251/AC-01).
- **AC-03** Der Platzhaltertext einer ungeladenen Stufe liest seine Schrift aus dem Katalog (B-198/AC-01).
- **AC-04** Die Schriftregel nennt eine Mindestgröße für die Mitspieler-Zelle (B-197/AC-01).
- **AC-05** Der Reiter sitzt beim Laufen und Sprinten auf dem Sattel, nicht auf der Kruppe (B-320/AC-01, B-320/AC-02).

## Offene Fragen

- B-197: Mindestgröße in der Mitspieler-Zelle, entscheidet 🧑.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GR7.1 Ganzzahlige Skalierung, Platzhaltertext aus dem Katalog (AC-02, AC-03, AC-04).
- GR7.2 Figuren-Kandidaten und Auswahl (🧑) (AC-01).
- GR7.3 Sattelsitz des Reiters beim Laufen und Sprinten (AC-05).
- GR7.4 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
