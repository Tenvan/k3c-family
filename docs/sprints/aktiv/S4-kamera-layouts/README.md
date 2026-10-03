# S4 · CLI · Kamera je Stufe und Layouts 1–4

- **Status:** aktiv
- **Domäne:** CLI
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-106
- **Start-Commit:** 605f467
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1; umfasst B-106

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

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| S4.1 | `S4.1-zelle-stufe.md` | Umsetzung | autonom | fertig |
| S4.2 | `S4.2-radar-hud-schrift.md` | Umsetzung | autonom | fertig |
| S4.3 | `S4.3-abnahme-geraet.md` | Workshop | Mensch | offen |
| S4.4 | `S4.4-review.md` | Review | autonom | offen |

## Abnahme

–
