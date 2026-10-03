# S7 · CLI · Monarch auf dem Standard-Reittier

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-173
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Simulation kennt nach S1 (B-152) ein Standard-Reittier je Monarch; der Client zeichnet den Monarchen noch als einzelne Figur. Die Reittier-Sprites und Sattelpunkte liegen in `data/sprites.json` › `mounts`.

## Ziel

Der Monarch erscheint beritten, mit Animationen für Stehen, Laufen und Sprint. Am Ende sichtbar: Zwei Spieler im Split-Screen reiten am TV über die Stufe.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy; 🧑 nimmt am Gerät ab.

## Anforderungen

B-173 › Anforderungen.

## Nicht-Ziele

Andere Reittiere als Auswahl, neue Grafiken (GR2), Reittiere für Truppen, Ton.

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots; Lizenz-Credits der Reittier-Sprites (CC-BY) stehen in `public/sprites/CREDITS.md`; Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Spieler läuft nach links → das Reittier ist gespiegelt, der Reiter sitzt am Sattelpunkt.

## Ausnahme- und Fehlerfälle

Unbekannter Sprite-Schlüssel → bisherige Figur und Log-Eintrag.

## Akzeptanzkriterien

- **AC-01** Auswahl von Sprite, Animation und Reiter-Position ist eine getestete reine Funktion (B-173/AC-01).
- **AC-02** Jeder Monarch erscheint mit Reittier, die Animationen unterscheiden sich je Bewegung (B-173/AC-02).
- **AC-03** Zwei Spieler im Split-Screen zeigen beide Reittiere korrekt, der Client rechnet nichts (B-173/AC-03).
- **AC-04** 🧑 hat die Darstellung am TV und am Handy abgenommen (B-173/AC-04).

## Offene Fragen

Standard-Tier (Pferd?): `docs/fragenkatalog.md Q23`, Auswahl 🧑.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- S7.1 Reine Funktion für Sprite, Animation und Sattelpunkt mit Test (AC-01).
- S7.2 Anbindung im Renderer, Split-Screen 1–4 (AC-02, AC-03).
- S7.3 🧑 Abnahme am TV und am Handy (AC-04).
- S7.4 Review des Sprints (Code-Sprint) (AC-01, AC-02, AC-03, AC-04).

## Abnahme

–
