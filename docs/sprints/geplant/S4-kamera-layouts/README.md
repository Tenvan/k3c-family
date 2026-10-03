# S4 · CLI · Kamera je Stufe und Layouts 1–4

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-106
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Split-Screen-Zellen zeigen dieselbe Welt (`src/scenes/GameScene.ts`, `worldRenderer.ts`, `layout.ts`); Radar und HUD kennen keine Stufe je Zelle. Im Viertel-Split werden Texte effektiv sehr klein (Plan Lücke 5). Voraussetzung: Stufe je Spieler im Protokoll (B-100, B-104) und die Mindest-Schriftgröße aus B-136.

## Ziel

Jede Zelle zeigt die Stufe ihres Spielers, auch bei Spielern in verschiedenen Stufen; Radar, HUD und die Layouts 1–4 folgen der Stufe der Zelle und halten die Mindest-Schriftgröße. Am Ende sichtbar: 2 Spieler am selben Gerät in verschiedenen Stufen.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy; 🧑 testet am Gerät.

## Anforderungen

B-106 › Anforderungen; zusätzlich Layouts 1–4 mit der Mindest-Schriftgröße je Viertel nach B-136 und Radar-Anpassung.

## Nicht-Ziele

Simulation (B-100), Protokoll (B-104), Skill-Menü und Overlay (S3).

## Regeln und Einschränkungen

`CLAUDE.md` (Client rechnet nichts), Layout 1–4. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`.

## Beispiele

Zwei Spieler am selben Bildschirm: oben Wald, unten Höhle, Radar je Zelle passend; im Viertel-Split sind alle Texte mindestens so groß wie die Regel verlangt.

## Ausnahme- und Fehlerfälle

Stufe noch nicht geladen → Platzhalter statt Absturz.

## Akzeptanzkriterien

- **AC-01** Zeichnen je Zelle aus der Stufe des Spielers; die reine Zuordnungs-Logik ist getestet (B-106/AC-01).
- **AC-02** Radar und HUD zeigen je Zelle die richtige Stufe (B-106/AC-02).
- **AC-03** Test: Die Schriftgröße je Layout 1–4 unterschreitet die Mindestgröße aus der Regel (B-136) nicht.
- **AC-04** 🧑 hat 2 Spieler in verschiedenen Stufen sowie das Viertel-Layout am Gerät abgenommen (B-106/AC-03).

## Offene Fragen

Mindest-Schriftgröße je Viertel: 🧑, `docs/fragenkatalog.md Q03`, Regel in B-136.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- S4.1 Zuordnung Spieler → Stufe → Zelle als reine Funktion, Zeichnen je Zelle mit Parallax und Palette (AC-01).
- S4.2 Radar und HUD je Zelle, Layouts 1–4 mit Mindestschrift (AC-02, AC-03).
- S4.3 🧑 Abnahme am Gerät mit 2 Spielern in verschiedenen Stufen und im Viertel-Split (AC-04).
- S4.4 Review des Sprints (Code-Sprint) (AC-01, AC-02, AC-03, AC-04).

## Abnahme

–
