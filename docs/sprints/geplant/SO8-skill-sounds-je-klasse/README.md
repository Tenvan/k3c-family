# SO8 · DEV, CLI · Skill-Sounds je Klasse

- **Status:** geplant
- **Projekt:** SND
- **Domäne:** DEV, CLI
- **Reife:** bereit
- **Tickets:** B-576
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-24, Chat (Ralf), Revision 1

## Ausgangslage

Alle Skills spielen beim Wirken keinen Ton; `sfx:skill` gehört nur zum Fund eines Skill-Punkts. Entscheidung 🧑 (B-558, KR4.1): Jede Klasse bekommt einen eigenen Skill-Sound (B-576). Der Client erkennt das Wirken schon (`SkillCasts`) und kennt die Linie, ein Protokollfeld ist nicht nötig.

## Ziel

Der Skill jeder Klasse (Tank, Zauberer, Heiler, Dieb) klingt eigen. Am Ende hörbar: Skill wirken im Spiel spielt den Klassen-Ton.

## Beteiligte und Zielgruppen

Spielende; Agent DEV und CLI arbeitet autonom; 🧑 bestätigt die Töne beim Hören.

## Anforderungen

B-576 › Anforderungen.

## Nicht-Ziele

Protokoll- und Server-Änderungen, die Auswahl der Töne für passive und einzelne Skills (später mit dem AI-Chat), Pegel-Feinschliff.

## Regeln und Einschränkungen

Client rechnet nichts (ADR 001); CC0 oder CC-BY mit Credit; Atlas nach Slot-Änderung neu; 2 Spieler gleichzeitig; Credits-Test grün.

## Beispiele

Heiler wirkt `groupHeal` → Heiler-Ton.

## Ausnahme- und Fehlerfälle

Linie ohne Slot oder Datei fehlt → `sfx:skill` als Rückfall (aktive Skills). Passiver Skill ohne gesetzten Slot → still.

## Akzeptanzkriterien

- **AC-01** Vier Slots `sfx:skill:<linie>` mit Kandidaten und Credit, Atlas neu (B-576/AC-01).
- **AC-02** Das Wirken eines aktiven Skills spielt den Ton seiner Linie, Rückfall `sfx:skill` (B-576/AC-02).
- **AC-03** Für passive und einzelne Skills lässt sich ein Ton über `sfx:skill:<skillId>` definieren, ohne Eintrag bleiben sie still (B-576/AC-03).
- **AC-04** 🧑 hat die vier Töne gehört und bestätigt oder geändert (B-576/AC-04).

## Offene Fragen

keine. Auswahl der Töne als Default durch den Agenten, 🧑 bestätigt in SO8.4. Spec-Freigabe steht aus.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SO8.1 | `SO8.1-klassen-slots-katalog.md` | Umsetzung | autonom | offen |
| SO8.2 | `SO8.2-ausloeser-skill-ton.md` | Umsetzung | autonom | offen |
| SO8.3 | `SO8.3-review.md` | Review | autonom | offen |
| SO8.4 | `SO8.4-hoerprobe-klassen-toene.md` | Workshop | Mensch | offen |

- SO8.1 (DEV) Vier Slots `sfx:skill:<linie>` in `data/assets.json` mit Kenney-CC0-Kandidaten, Credits, `task gen:soundatlas` (AC-01).
- SO8.2 (CLI) Auslöser: Abbildung Skill → (Skill-Slot vor Linien-Slot vor `sfx:skill`) in `src/audio` und `SkillCasts`-Anbindung für aktive Skills und neu gelernte passive, Vitest (AC-02, AC-03).
- SO8.3 Review: alle Kriterien prüfen.
- SO8.4 (Mensch) Hörprobe der vier Klassen-Töne am PC (AC-04).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
