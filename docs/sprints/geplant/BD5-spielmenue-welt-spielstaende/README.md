# BD5 · SRV · Spielmenü zeigt die Welt, legt Spielstände an und löscht sie

- **Status:** geplant
- **Projekt:** BED
- **Domäne:** SRV
- **Reife:** Entwurf
- **Tickets:** B-506
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Aus der Abnahme WZ18.8 (🧑 am PC, 2026-10-20): Welt anlegen, im Spiel starten und Inselwechsel funktionieren, aber das Spielmenü zeigt die gewählte oder verfügbare Welt nicht und kann Spielstände weder neu anlegen noch löschen (B-506). Ein vierter Punkt der Meldung war leer.

## Ziel

Spielende sehen im Spielmenü, in welcher Welt sie spielen, und verwalten ihre Spielstände dort selbst. Am Ende sichtbar: Weltname im Menü, neuer Spielstand und Löschen mit Rückfrage, am PC von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Spielende mit Controller und Tastatur; Agent in SRV und CLI; 🧑 klärt Umfang und nimmt ab.

## Anforderungen

B-506 › Ausgangslage (Spec noch Entwurf, Anforderungen nach Klärung der Offenen Fragen ausformulieren).

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

🧑: Welcher vierte Punkt war gemeint (Meldung leer)? Welt-Auswahl beim Neuanlegen oder nur Anzeige der Welt je Spielstand? Ist mit „Spielmenü“ das Menü im Spiel oder die Lobby gemeint?

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BD5.1 (SRV) Spielstände mit Welt anlegen und löschen, ggf. Protokoll (AC-01).
- BD5.2 (CLI) Spielmenü: Welt anzeigen, Spielstand anlegen und löschen (AC-02).
- BD5.3 (CLI) Review (Code-Sprint): alle Kriterien prüfen.
- BD5.4 (CLI, Mensch) Abnahme am PC (AC-03).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
