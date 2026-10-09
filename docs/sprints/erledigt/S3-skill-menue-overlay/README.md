# S3 · CLI · Skill-Menü, Tasten und Aktionen-Overlay

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-124, B-125
- **Start-Commit:** d00f168
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1, durch 🧑; umfasst B-124, B-125

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

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| S3.1 | `S3.1-eingabe-slots.md` | Umsetzung | autonom | fertig |
| S3.2 | `S3.2-skill-menue.md` | Umsetzung | autonom | fertig |
| S3.3 | `S3.3-aktionen-overlay.md` | Umsetzung | autonom | fertig |
| S3.4 | `S3.4-abnahme-geraet.md` | Workshop | Mensch | fertig |
| S3.5 | `S3.5-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-10-05, Review S3.5 (Diff `src/` gelesen, `task check` und `task check:go` grün): keine schweren Befunde (B unbelegt, View + Menu unverändert, kein `Math.random()`, Menüs und Overlays je Slot getrennt).
- **AC-01 bis AC-04** nachgewiesen in S3.1 (Slot-Belegung getestet), S3.2 (Menü, Tests für 2 Spieler, Browser-Pane), S3.3 (Aktion → Taste/Text getestet, Overlay im Pane mit 2 Spielern); Wiederbeleben-Teil von AC-04 verschoben (B-120).
- **AC-05** angenommen, Validierung offen (S3.4); S3.4 steht im Fahrplan unter „Offen am Gerät“, B-124 und B-125 bleiben bis dahin offen.
- Neue Tickets: B-285 (lernbare Skills im Protokoll, entstand in S3.2).
- Version: v0.7.0 gesetzt (2026-10-05, nach Bestätigung durch 🧑; `task check:all` grün)
- 2026-10-07: Sprint auf Entscheidung 🧑 abgeschlossen. Die offene Abnahme am Gerät ist nicht durchgeführt (verworfen) und geht in die Gesamtprüfung B-337/AC-05 über; keine weiteren Anzeige- und Touch/Tasten-Abnahmen bis zur Umsetzung von B-337.
