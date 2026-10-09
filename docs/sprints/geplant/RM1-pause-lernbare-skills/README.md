# RM1 · SRV · Raum-Pause im Couch-Raum und lernbare Skills vom Server

- **Status:** geplant
- **Projekt:** SKL
- **Domäne:** SRV
- **Reife:** Entwurf
- **Tickets:** B-285
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

Pause hält den Raum nicht an (B-214); der Client weiß nicht, welche Skills der Server annimmt (B-285).

## Ziel

Ein Couch-Raum pausiert wirklich, online ist ein pausierter Monarch geschützt, und das Skill-Menü zeigt die lernbaren Skills vom Server. Am Ende sichtbar: Pause hält den Couch-Raum an; Skill-Menü zeigt nur, was der Server annimmt.

## Beteiligte und Zielgruppen

Spielende am TV und Handy; Agent baut Server und Protokoll.

## Anforderungen

B-285 › Anforderungen. (B-214 ist seit PJ3 ohne Sprint im Projekt BED.)

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

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- RM1.1 entfällt (Pause im Couch-Raum, B-214 an BED, PJ3).
- RM1.2 Lernbare Skills je Spieler im Protokoll (AC-02).
- RM1.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
