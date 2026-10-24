# GR7 · CLI · Figuren-Lücken, ganzzahlige Skalierung und Schrift

- **Status:** erledigt
- **Projekt:** GRA
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-193, B-251, B-198, B-197, B-320
- **Start-Commit:** 46c2c435
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-12, Chat (Ralf), Revision 1

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

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GR7.1 | `GR7.1-schrift-platzhalter-partner.md` | Umsetzung | autonom | fertig |
| GR7.2 | `GR7.2-sattelsitz.md` | Umsetzung | autonom | fertig |
| GR7.3 | `GR7.3-figuren-kandidaten.md` | Workshop | Mensch | fertig |
| GR7.4 | `GR7.4-skalierung-tv.md` | Umsetzung | Mensch | fertig |
| GR7.5 | `GR7.5-review.md` | Review | autonom | fertig |

- GR7.1 Platzhaltertext aus dem Katalog, Regel für die Mitspieler-Zelle (AC-03, AC-04).
- GR7.2 Sattelsitz des Reiters beim Laufen und Sprinten (AC-05).
- GR7.3 Figuren-Kandidaten und Auswahl, 🧑 wählt (AC-01).
- GR7.4 Figuren am TV beobachten, ganzzahlig skalieren oder verwerfen, 🧑 beobachtet (AC-02).
- GR7.5 Review (Code-Sprint): alle Kriterien prüfen; GR7.3 und GR7.4 sind keine Abhängigkeit (Hardware entkoppelt).

## Abnahme

2026-10-24: GR7.1, GR7.2, GR7.5 früher erledigt; GR7.3 (Figuren eingebaut, Mängel B-593 verworfen) und GR7.4 (keine Umstellung, B-251 verworfen) auf Entscheid von Ralf: Alle Figuren und Grafiken gelten als abgenommen, bisherige Umsetzungen sind ausreichend, Grafik wird künftig mit dem AI-Chat gelöst. B-193, B-329, B-331 erledigt. Nicht einzeln am Gerät geprüft.
