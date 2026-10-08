# SO2 · CLI · SFX-Katalog und Einbau

- **Status:** geplant
- **Projekt:** SND
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-167
- **Start-Commit:** –
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, Revision 1; mit Änderungen aus der Spec-Prüfung

## Ausgangslage

Nach SO1 steht der Audio-Kern, es gibt aber keine Effekt-Sounds und keinen Katalog. Details in B-167.

## Ziel

Jedes wichtige Ereignis hat einen Sound mit Quelle und Lizenz, und die Sounds sind eingebaut. Am Ende sichtbar: `docs/assets/sounds.md`, Dateien mit Credits unter `public/audio/`, hörbare Effekte am TV.

## Beteiligte und Zielgruppen

🧑 wählt Quellen und Stil (Q15); der Agent katalogisiert und baut ein; Spieler am TV.

## Anforderungen

B-167 › Anforderungen.

## Nicht-Ziele

Audio-Kern (SO1), Musik (SO4), Hörprobenseite (SO3), eigene Kompositionen.

## Regeln und Einschränkungen

CC0 oder CC-BY mit Credits (B-165-Test); `src/scenes` rechnet nichts; 2 Spieler; Voraussetzung: SO1, F3, F4.

## Beispiele

Münze aufheben → Münz-Sound; Datei fehlt → Ereignis bleibt stumm.

## Ausnahme- und Fehlerfälle

Sound lädt nicht → stumm und Log-Eintrag, kein Absturz.

## Akzeptanzkriterien

- **AC-01** `docs/assets/sounds.md` enthält jedes Ereignis der Liste mit Sound, Quelle, Lizenz oder als Lücke mit Ticket (B-167/AC-01).
- **AC-02** Jede Sound-Datei unter `public/audio/` hat einen Credit-Eintrag (B-167/AC-02).
- **AC-03** Münze aufheben, Schlag, Gegner-Tod, Bauen fertig und Nacht naht lösen am TV je ihren Sound aus (B-167/AC-03).
- **AC-04** Das Mapping Ereignis → Sound ändert keinen Spielzustand (B-167/AC-04).
- **AC-05** Eine fehlende Sound-Datei lässt das Spiel ohne Fehlermeldung weiterlaufen (B-167/AC-05).
- **AC-06** `task check` ist grün.
- **AC-07** Nacht leise: Nachts spielen die Effekte gedämpft (Q15) (B-167/AC-06).

## Offene Fragen

- Hub-Ausbau, Boss-Auftritt und UI-Klicks: Woher kommen die Auslöser (kein Feedback-Event in Q08)? Bis dahin nicht in der Ereignisliste (B-167 › Offene Fragen).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SO2.1 | `SO2.1-workshop-auswahl.md` | Workshop | Mensch | offen |
| SO2.2 | `SO2.2-sounds-beschaffen.md` | Umsetzung | autonom | offen |
| SO2.3 | `SO2.3-einbau-rueckfall.md` | Umsetzung | autonom | offen |
| SO2.4 | `SO2.4-review.md` | Review | autonom | offen |
| SO2.5 | `SO2.5-hoerprobe-tv.md` | Workshop | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
