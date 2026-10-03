# S5 · CLI · Optionen- und Pause-Szene

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-146
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Aktion `pause` existiert in `src/input/playerInput.ts`, aber keine Szene; es gibt keine getrennten Lautstärken, keinen Schalter für Screenshake oder Flash und keine Farbschwäche-Symbole. Voraussetzung: Pause-Regel aus B-135 (F1). Das Speichern beim Verlassen ist Serverarbeit (S2, B-147).

## Ziel

Eine Szene für Optionen und Pause ist mit Controller, Tastatur und Touch bedienbar und speichert Lautstärke Musik/SFX, Screenshake aus, Flash aus und Farbschwäche-Symbole je Gerät. Am Ende sichtbar: Einstellungen bleiben nach dem Neuladen erhalten.

## Beteiligte und Zielgruppen

Spieler am TV, Handy und PC; 🧑 nimmt am Gerät ab.

## Anforderungen

B-146 › Anforderungen.

## Nicht-Ziele

Audio-Mixer (B-011), Juice-Effekte (B-164), Controller-Glyphen (S6), Spielstand speichern (S2).

## Regeln und Einschränkungen

`CLAUDE.md` (View + Menu reserviert, B nie belegen, Home-Button oben, Client rechnet nichts). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`.

## Beispiele

Spieler öffnet mit Menu die Pause, stellt Musik auf 0 %, schaltet Screenshake aus → nach dem Neuladen gelten die Werte noch.

## Ausnahme- und Fehlerfälle

`localStorage` gesperrt → Standardwerte, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Die Einstellungen liefern Standardwerte, begrenzen und fallen bei kaputtem Speicher zurück (B-146/AC-01).
- **AC-02** Die Szene ist mit Controller, Tastatur und Touch bedienbar und stellt alle Optionen getrennt ein (B-146/AC-02).
- **AC-03** Die Einstellungen bleiben nach Neuladen erhalten (B-146/AC-03).
- **AC-04** View + Menu führt weiter zur Landingpage, B bleibt unbelegt (B-146/AC-04).
- **AC-05** 🧑 hat die Szene am TV und am Handy abgenommen (B-146/AC-05).

## Offene Fragen

Pause im gemeinsamen Raum: 🧑, `docs/fragenkatalog.md Q01`.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- S5.1 Einstellungen als reine Funktionen und Speicher je Gerät mit Test (AC-01, AC-03).
- S5.2 Szene Optionen/Pause, Bedienung mit allen Geräten, Pause-Regel aus B-135 umsetzen (AC-02, AC-04).
- S5.3 🧑 Abnahme am TV und am Handy (AC-05).
- S5.4 Review des Sprints (Code-Sprint) (AC-01, AC-02, AC-03, AC-04, AC-05).

## Abnahme

–
