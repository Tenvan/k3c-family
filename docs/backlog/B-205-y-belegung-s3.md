# B-205 · Die Y-Belegung in S3 folgt dem Beschluss „kein Bau-Menü“

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** S8
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Beschluss Q34 mit Folgebeschluss zur Y-Taste (2026-10-04, `docs/fragenkatalog.md`): Es gibt **kein Bau-Menü** (Regel `docs/rules/materialien-gebaeude.md` § 3), Q06 ist angepasst, **Y ist frei** (für später). Die freigegebene Session `docs/sprints/geplant/S3-skill-menue-overlay/S3.1-eingabe-slots.md` (Zeile 17, „Belegung (beschlossen, Q06 …)“) nennt noch „Bau-Menü Y“. Im Code gibt es die Aktion `build` auf Y (Tastatur B) schon (`src/input/playerInput.ts`, Zeilen 10, 34, 92), ohne Nutzer in `src/scenes/` (per Suche geprüft). Die S3-Spec ist freigegeben (2026-10-03) und wird nicht still geändert.

## Ziel

S3 belegt Y nicht mit einem Bau-Menü; Planung, Regel und Code stimmen überein.

## Beteiligte und Zielgruppen

Spieler (Controller, Tastatur); Entwickler (CLI/PLAT in S3); 🧑 gibt die Änderung der Session frei.

## Anforderungen

- S3.1 nennt in der Belegung kein Bau-Menü; Y bleibt unbelegt (Beschluss Q34/Q06, 2026-10-04).
- Kein Bau-Menü in `src/input/` oder den Szenen: Die Aktion `build` (Y, Tastatur B) wird entfernt; die Zeile „Bau-Menü“ in `docs/game-design.md` › Steuerung ist bereits gestrichen.

## Nicht-Ziele

Neue Belegung für Y (später, eigenes Ticket); übrige Belegung aus Q06.

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › SDD: Änderung einer freigegebenen Spec nur über Revision und neue Freigabe durch 🧑. Controller-Taste B nicht belegen, View + Menu reserviert (`CLAUDE.md`).

## Beispiele

S3.1 wird umgesetzt → Y löst nichts aus; das Aktionen-Overlay (B-125) zeigt keine Y-Aktion.

## Ausnahme- und Fehlerfälle

S3.1 ist bereits umgesetzt, bevor die Session angepasst ist → Y-Belegung als Abweichung im Ergebnis melden und entfernen.

## Akzeptanzkriterien

- **AC-01** `grep -rn "Bau-Menü" docs/sprints/geplant/S3-skill-menue-overlay` findet keine Belegung von Y.
- **AC-02** Nach S3 belegt `src/input/` die Taste Y nicht.
- **AC-03** `task test -- planning` grün.

## Offene Fragen

Entschieden 2026-10-06 (🧑, Chat): redaktionelle Korrektur ohne neue S3-Revision; S8 korrigiert Code und den S3-Text (S3-Dateien sind erlaubte Planungs-Dateien der Session S8.2).

## Notizen

Entstanden aus dem Folgebeschluss zu Q34 (Fragenkatalog Block 5).
