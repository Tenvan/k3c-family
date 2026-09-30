# X1 · PLAT · Xbox-Machbarkeit

- **Status:** geplant
- **Domäne:** PLAT
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-006, B-026
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Gamepad-Testseite ist fertig, der Test auf der Xbox steht aus; die Steuerungstabelle beruht auf Vermutungen.

## Ziel

Die Steuerung auf der Xbox ist geprüft. Am Ende sichtbar: Bericht in `reports/`, Steuerungstabelle in `game-design.md` ohne „vermutlich“.

## Beteiligte und Zielgruppen

🧑 testet an der Xbox mit zwei Controllern; der Agent wertet aus.

## Anforderungen

B-006 und B-026 › Anforderungen.

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

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- X1.1 🧑 Gamepad-Test auf der Xbox (Anleitung im README), zwei Controller (AC-01).
- X1.2 Auswertung: Steuerungstabelle, Skill-Tasten (B-026), Sprite-Budget, HTTPS ja/nein (AC-02, AC-03).
- X1.3 🔍 Review (alle).

## Abnahme

–
