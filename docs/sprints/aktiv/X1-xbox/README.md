# X1 · PLAT · Xbox-Machbarkeit

- **Status:** aktiv
- **Domäne:** PLAT
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-006, B-026, B-166
- **Start-Commit:** 6c8ba2a
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1; umfasst B-006, B-026, B-166 und die Domänen-Ausnahme

## Ausgangslage

Die Gamepad-Testseite ist fertig, der Test auf der Xbox steht aus; die Steuerungstabelle beruht auf Vermutungen. Die Seite kennt kein Audio; ob Edge auf der Xbox Töne nach einer Geste abspielt und welche Formate er dekodiert, ist offen (B-166), blockiert SO1.

## Ziel

Die Steuerung und der Ton auf der Xbox sind geprüft. Am Ende sichtbar: Bericht in `reports/` (mit Audio-Ergebnis), Steuerungstabelle in `game-design.md` ohne „vermutlich“, Audio-Ergebnis in `game-design.md`.

## Beteiligte und Zielgruppen

🧑 testet an der Xbox mit zwei Controllern; der Agent wertet aus.

## Anforderungen

B-006, B-026 und B-166 › Anforderungen.

## Nicht-Ziele

Umbau der Eingabe für mehrere lokale Spieler (SP08).

## Regeln und Einschränkungen

Den Test an der Xbox macht nur 🧑. B nicht belegen, View + Menu reserviert. **Domänen-Ausnahme (Freigabe dieser Spec erlaubt sie, wie bei SP11):** X1.1 darf Testdateien unter `public/audio-test/` anlegen (CLI), X1.3 darf `docs/game-design.md` (REG) nachführen; beides verlangen B-006 und B-166.

## Beispiele

Test mit zwei Controllern → Bericht in `reports/`, daraus Steuerungstabelle und Sprite-Budget.

## Ausnahme- und Fehlerfälle

Die Xbox erreicht den Server nicht → HTTPS und Netz nach README prüfen, Befund notieren.

## Akzeptanzkriterien

- **AC-01** Ein Bericht der Xbox liegt in `reports/` (B-006/AC-01).
- **AC-02** Die Steuerungstabelle enthält kein „vermutlich“ mehr, inklusive Skill-Tasten (B-006/AC-02, B-026/AC-01).
- **AC-03** Sprite-Budget und „HTTPS ja/nein“ stehen in `game-design.md`.
- **AC-04** Die Gamepad-Testseite prüft Audio: Zustand des `AudioContext`, Abspielversuch ohne Geste, Formate ogg, m4a, mp3, wav, Latenz; die Auswertung schreibt das Feld `audio` (B-166/AC-01, B-166/AC-02, B-166/AC-03, B-166/AC-04).
- **AC-05** Ein Bericht der Xbox mit Audio-Ergebnis liegt in `reports/`, Format- und Autoplay-Ergebnis stehen in `game-design.md` (B-166/AC-05).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| X1.1 | `X1.1-audio-testseite.md` | Umsetzung | autonom | fertig |
| X1.2 | `X1.2-xbox-test.md` | Workshop | Mensch | in Arbeit |
| X1.3 | `X1.3-auswertung.md` | Umsetzung | autonom | offen |

## Abnahme

–
