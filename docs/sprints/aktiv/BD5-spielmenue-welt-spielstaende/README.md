# BD5 · SRV, CLI · Spielmenü zeigt die Welt, legt Spielstände an und löscht sie

- **Status:** aktiv
- **Projekt:** BED
- **Domäne:** SRV, CLI
- **Reife:** bereit
- **Tickets:** B-506
- **Start-Commit:** 013dcb34
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Aus der Abnahme WZ18.8 (🧑 am PC, 2026-10-20): Welt anlegen, im Spiel starten und Inselwechsel funktionieren, aber das Spielmenü zeigt die gewählte oder verfügbare Welt nicht und kann Spielstände weder neu anlegen noch löschen (B-506). Ein vierter Punkt der Meldung war leer.

## Ziel

Spielende sehen im Spielmenü, in welcher Welt sie spielen, und verwalten ihre Spielstände dort selbst. Am Ende sichtbar: Weltname im Menü, neuer Spielstand und Löschen mit Rückfrage, am PC von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Spielende mit Controller und Tastatur; Agent in SRV und CLI; 🧑 klärt Umfang und nimmt ab.

## Anforderungen

- **Spielmenü** ist die Lobby (Raumliste mit „Spielen“, Räumen und Spielständen); im Pausenmenü im Spiel steht zusätzlich die Welt des Raums (Default, Entscheidung B-534).
- Die Lobby zeigt die Welt, in der ein neues Spiel startet (`?world=`, sonst `standard`), und je Spielstand dessen Welt (`GET /api/saves` nennt `world`).
- „Neuer Spielstand“ legt einen frischen Stand unter dem nächsten freien Namen `spiel-N` in dieser Welt an und startet ihn (`create` mit `fresh: true`, keine Protokoll-Änderung).
- Löschen eines Spielstands: Pad X oder Taste Entf auf einem Spielstand öffnet eine Rückfrage mit vorgewähltem „Abbrechen“; nur „Löschen“ + A/Enter löscht genau diesen einen Stand. Kein Massenlöschen.
- Server: `DELETE /api/save?slot=NAME` (HTTP, Protokoll-Version bleibt 8). Der Stand wandert in seine rotierenden Sicherungen (wiederherstellbar über `/api/save/restore`), die Sicherungen bleiben (Entscheidung B-533). Ist er in einem Raum offen → `409`, Hinweis im Menü.
- Texte zentral in `texts.de.ts`/`texts.en.ts`; Taste B bleibt frei, View + Menu reserviert.

## Nicht-Ziele

Welten anlegen oder bearbeiten im Spiel (Asset-Studio, WZ18); Wiederherstellen gelöschter Stände; HUD-Leiste und Austreten (BD4).

## Regeln und Einschränkungen

Reihenfolge SRV → CLI; braucht das Löschen eine Protokoll-Änderung, eine eigene Session für beide Enden. Löschen nur mit Rückfrage, Sicherungen bleiben (Rotation, F4). Taste B nicht belegen; View + Menu reserviert. Spieltexte zentral und zweisprachig (de/en).

## Beispiele

Spiel in Welt `nordmeer` → Menü zeigt „Welt: Nordmeer“. „Neuer Spielstand“ → Name aus Seed, Spiel startet frisch. „Löschen“ → Rückfrage, danach ist der Stand weg aus der Liste.

## Ausnahme- und Fehlerfälle

Stand ist in einem offenen Raum geladen → Löschen abgelehnt mit Hinweis. Server lehnt ab → Meldung im Menü, nichts still. Welt einer Spielstand-Datei fehlt → Hinweis statt Absturz.

## Akzeptanzkriterien

- **AC-01** Der Server bietet Anlegen und Löschen von Spielständen mit Welt an, Löschen eines geladenen Stands wird abgelehnt (Go-Test, B-506).
- **AC-02** Das Spielmenü zeigt die Welt und legt Spielstände an und löscht sie mit Rückfrage (Vitest der Menü-Logik, B-506).
- **AC-03** 🧑 hat das Spielmenü am PC abgenommen (B-506).

## Offene Fragen

Autonom entschieden (Ralf: „autonom abarbeiten“), im Nachgang zu bestätigen:
- Vierter Punkt der Meldung: unbekannt, bleibt außen vor.
- Welt-Auswahl beim Neuanlegen: nein, nur Anzeige (andere Welten gibt es nur im Dev-Mode über `?world=`), B-534.
- „Spielmenü“ = Lobby, plus Weltanzeige im Pausenmenü, B-534.
- Löschen per HTTP ohne Token, als Sicherung behalten statt endgültig, B-533.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BD5.1 | `BD5.1-server-spielstaende-loeschen.md` | Umsetzung | autonom | fertig |
| BD5.2 | `BD5.2-spielmenue-welt-anlegen-loeschen.md` | Umsetzung | autonom | offen |
| BD5.3 | `BD5.3-review.md` | Review | autonom | offen |
| BD5.4 | `BD5.4-abnahme-pc.md` | Workshop | Mensch | offen |

- BD5.1 (SRV) Spielstände mit Welt auflisten und löschen, HTTP (AC-01).
- BD5.2 (CLI) Spielmenü: Welt anzeigen, Spielstand anlegen und löschen (AC-02).
- BD5.3 (CLI) Review (Code-Sprint): alle Kriterien prüfen.
- BD5.4 (CLI, Mensch) Abnahme am PC (AC-03).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
