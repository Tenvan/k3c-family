# S3 · CLI · Skill-Menü, Tasten und Aktionen-Overlay

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-124, B-125
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Client kennt weder Schlag, Skill-Slots noch Skill-Menü; Hinweise sind Texte am Bildschirmrand. Voraussetzung: S2 (Protokoll) und X1 (Tastenbelegung am Controller, B-026, Domäne PLAT, nicht in diesem Sprint).

## Ziel

Schlag, Skill-Slots 1–4 und Skill-Menü sind mit Controller, Tastatur und Touch bedienbar; gültige Aktionen erscheinen als Overlay am Ort mit der Taste des zuletzt benutzten Geräts. Am Ende sichtbar: 🧑 spielt am Gerät Schlag, Skill, Punkte verteilen und liest die Aktionen im Overlay.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy; 🧑 nimmt am Gerät ab.

## Anforderungen

B-124 und B-125 › Anforderungen.

## Nicht-Ziele

Simulation (S1), Protokoll (S2), Controller-Glyphen (S6, B-149), Optionen/Pause (S5), Kamera je Stufe (S4).

## Regeln und Einschränkungen

`CLAUDE.md` (B nie belegen, View + Menu reserviert, Home-Button oben, Client rechnet nichts); Eingabe nur über `PlayerInput`; `src/input/` ist PLAT, die Freigabe der Spec erlaubt die Änderung (B-124 › Offene Fragen). Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Spieler drückt LB → Skill 1 feuert, die Abklingzeit läuft im HUD; an der Burg steht „A halten: Hub-Stufe 2“.

## Ausnahme- und Fehlerfälle

Skill ohne Punkte → Taste ohne Wirkung, Hinweis im Overlay; mehrere gültige Aktionen am selben Ort → die wichtigste zuerst.

## Akzeptanzkriterien

- **AC-01** Reine Funktion für die Slot-Belegung je Gerät ist getestet (B-124/AC-01).
- **AC-02** Skill-Menü und Slots sind mit Controller, Tastatur und Touch bedienbar (B-124/AC-02).
- **AC-03** Reine Funktion Aktion → Text und Symbol je Gerät ist getestet (B-125/AC-01).
- **AC-04** Das Overlay zeigt die Aktionen am Ort für Bauen, Zahlen, Wiederbeleben und Schlag (B-125/AC-02).
- **AC-05** 🧑 hat Slots, Menü, Tasten und Overlay am Gerät abgenommen (B-124/AC-03, B-125/AC-03).

## Offene Fragen

Skill-Tasten am Controller (LB/RB bestätigen): 🧑, `docs/fragenkatalog.md Q06`, vor Sprintstart über X1.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- S3.1 Eingabe: Schlag, Skill-Slots, Skill-Menü in `src/input/` und `touchInput.ts` mit Slot-Belegung als reine Funktion (AC-01, AC-02).
- S3.2 Skill-Menü in der HUD-Szene: Punkte verteilen, Respec, Slots und Abklingzeiten (AC-02).
- S3.3 Aktionen-Overlay aus Snapshot-Daten, Funktion Aktion → Text/Symbol (AC-03, AC-04).
- S3.4 🧑 Abnahme am Gerät (Controller, Tastatur, Touch) (AC-05).
- S3.5 Review des Sprints (Code-Sprint) (AC-01, AC-02, AC-03, AC-04, AC-05).

## Abnahme

–
