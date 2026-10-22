# RM1 · SRV · Raum-Pause im Couch-Raum und lernbare Skills vom Server

- **Status:** aktiv
- **Projekt:** SKL
- **Domäne:** SRV
- **Reife:** bereit
- **Tickets:** B-285, B-495
- **Start-Commit:** 17fc90d1
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-10-22, Revision 2

## Ausgangslage

Pause hält den Raum nicht an (B-214); der Client weiß nicht, welche Skills der Server annimmt (B-285).

## Ziel

Ein Couch-Raum pausiert wirklich, online ist ein pausierter Monarch geschützt, und das Skill-Menü zeigt die lernbaren Skills vom Server. Am Ende sichtbar: Pause hält den Couch-Raum an; Skill-Menü zeigt nur, was der Server annimmt.

## Beteiligte und Zielgruppen

Spielende am TV (Xbox) und am PC; Agent baut Server und Protokoll.

## Anforderungen

B-285 › Anforderungen; B-495 › Anforderungen (ergänzt 2026-10-21 beim Einplanen: Aktionsliste nennt `respec` nach `sim.CanRespec`). (B-214 ist seit PJ3 ohne Sprint im Projekt BED.)

## Nicht-Ziele

Tier-Gating im Client nachrechnen, neue Skills.

## Regeln und Einschränkungen

SRV; Protokoll-Änderung als eigene Session für beide Enden. Voraussetzung: B-270 (SK1) für die Abfrage der Lern-Regeln.

## Beispiele

Couch-Raum, Menü auf → Tick steht; online → Monarch unverwundbar, höchstens die festgelegte Frist.

## Ausnahme- und Fehlerfälle

Pause im gemischten Raum → nur Schutz des Monarchen, kein Anhalten.

## Akzeptanzkriterien

- **AC-01** entfällt, B-214 an BED (PJ3, 2026-10-08). Vorher: Der Server pausiert den Raum im Couch-Raum und schützt den stehenden Monarchen online (B-214/AC-01, B-214/AC-02).
- **AC-02** Der Server nennt je Spieler die lernbaren Skills (B-285/AC-01, B-285/AC-02).
- **AC-03** Die Aktionsliste nennt `respec` genau dann, wenn `sim.CanRespec` es erlaubt (B-495/AC-01, B-495/AC-02).

## Offene Fragen

keine. Entschieden (autonom, Claude, 2026-10-22): Die lernbaren Skills stehen als Feld `skills` im Eintrag `{ "action": "learn", "skills": [...] }` der Aktionsliste (B-285, Offene Frage); kein eigenes Feld, keine neue Protokoll-Version (Feld ist zusätzlich, ein älterer Server sendet `learn` ohne `skills`, der Client fällt dann auf das pauschale Lernen zurück). Bestätigung durch 🧑: Frage-Ticket im Sprint KR5.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RM1.2 | `RM1.2-lernbare-skills-protokoll.md` | Umsetzung | autonom | fertig |
| RM1.4 | `RM1.4-respec-in-aktionsliste.md` | Umsetzung | autonom | fertig |
| RM1.3 | `RM1.3-review.md` | Review | autonom | offen |
| RM1.1 | `RM1.1-pause-couch-raum.md` | Umsetzung | autonom | verworfen |

RM1.1 entfällt (Pause im Couch-Raum, B-214 an BED, PJ3).

## Abnahme

–
