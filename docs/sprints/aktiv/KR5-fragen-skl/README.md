# KR5 · REG · Kritische Fragen zu SKL

- **Status:** aktiv
- **Projekt:** KRT
- **Domäne:** REG
- **Reife:** bereit
- **Tickets:** B-559, B-560, B-561
- **Start-Commit:** 14dbee4a
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

## Ausgangslage

Das Projekt SKL ist autonom so weit umgesetzt, wie es ohne Ralf geht: RM1 erledigt (RM1.2, RM1.4, RM1.3), SK2.2, SK2.3, SK2.4 fertig. Dabei hat Claude drei Fragen mit Default vorentschieden (B-559 bis B-561). Offen bei Ralf bleiben der Workshop SK2.1 und die TV-Abnahme SK2.5; sie bleiben im Sprint SK2.

## Ziel

Ralf hat alle kritischen Fragen zu SKL an einer Stelle entschieden. Am Ende sichtbar: kein offenes Frage-Ticket im Projekt SKL, Folgearbeit mit klarer Entscheidung.

## Beteiligte und Zielgruppen

Entscheidet: 🧑 Ralf. Vorbereitet: Claude. Betroffen: Spielende am TV (Skill-Menü, Skill-Ringe, Startverteilung).

## Anforderungen

Je Ticket `B-5NN › Anforderungen` (Optionen, Default, Folgen):

| Ticket | Thema | Default gilt |
|---|---|---|
| B-559 | Form der lernbaren Skills im Protokoll (RM1.2) | Feld `skills` im Eintrag `learn` |
| B-560 | Dieb-Skills bauen oder Preset leer (SK2.2) | Preset leer, Dieb später |
| B-561 | Darstellung der Skills (SK2.3) | Ring je Klasse, ohne neue Ereignisse |

B-560 und B-561 am besten zusammen mit SK2.1 und SK2.5 am TV.

## Nicht-Ziele

Umsetzung der Folgearbeit (eigene Tickets/Sprints im Projekt SKL); die TV-Abnahme selbst (SK2.5); Studio-Seite (B-562).

## Regeln und Einschränkungen

Nur Ralf bestätigt; bis dahin gelten die Defaults. Dieb bauen oder Effekt je Skill sind Regel- bzw. Geschmacksentscheidungen.

## Beispiele

B-559 bestätigt → erledigt. B-560 „Dieb bauen“ → neuer SIM-Sprint im Projekt SKL (Backstab, Smoke Bomb, Golden, BAL-Lauf).

## Ausnahme- und Fehlerfälle

B-561 hängt an der TV-Abnahme SK2.5 → Ticket bleibt eingeplant, KR5.1 bleibt offen.

## Akzeptanzkriterien

- **AC-01** Jedes der Tickets B-559 bis B-561 trägt unter Notizen Ralfs Entscheidung.
- **AC-02** Bestätigte Tickets sind erledigt; zu jeder geänderten Entscheidung existiert ein Umsetzungs-Ticket oder Sprint im Projekt SKL.

## Offene Fragen

keine (die Fragen sind der Inhalt der Tickets; Defaults gelten).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KR5.1 | `KR5.1-fragen-skl.md` | Workshop | Mensch | fertig |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
