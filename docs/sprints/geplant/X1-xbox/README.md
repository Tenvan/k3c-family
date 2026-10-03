# X1 · PLAT · Xbox-Machbarkeit

- **Status:** geplant
- **Domäne:** PLAT
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-006, B-026, B-166
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Den Test an der Xbox macht nur 🧑. B nicht belegen, View + Menu reserviert.

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

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- X1.1 Audio-Abschnitt auf der Gamepad-Testseite (`gamepad-test.html`, `src/tools/gamepadTest.ts`) mit Testdateien und Test der Auswertung; autonom, vor dem Xbox-Test (AC-04).
- X1.2 🧑 Gamepad- und Audio-Test auf der Xbox (Anleitung im README), zwei Controller (AC-01, AC-05).
- X1.3 Auswertung: Steuerungstabelle, Skill-Tasten (B-026), Sprite-Budget, HTTPS ja/nein, Audio-Ergebnis; schließt den Sprint ab (Doku-Sprint, kein Review) (AC-02, AC-03, AC-05).

## Abnahme

–
