# U3 · PLAT · Level-Betrachter

- **Status:** aktiv
- **Domäne:** PLAT
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-092
- **Start-Commit:** 4d824fa
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02 🧑 Chat („ja, freigeben und umsetzen“; Revision 1)

## Ausgangslage

Es gibt Testseiten für Szenarien, Figuren und Grafiken, aber keine, die ein generiertes Level zeigt. Der Endpunkt dafür entsteht in U2 (B-092, B-091).

## Ziel

Die Seite `leveltest.html` zeigt Seed und Biom als Level-Streifen mit Warnungen und startet das Level im Spiel und ist von der Testseite (`testing.html`) aus erreichbar. Am Ende sichtbar: auf dem TV von der Testseite die Kachel „Level-Betrachter“ öffnen, mit Controller Seed ändern, scrollen und „Im Spiel starten“.

## Beteiligte und Zielgruppen

Entwickler, 🧑 beim Balancing; Bedienung mit Tastatur und Controller.

## Anforderungen

B-092 › Anforderungen.

## Nicht-Ziele

Spielen, Level bearbeiten oder speichern; Start mit beliebigem Seed oder in Höhle und Mine (B-092 › Nicht-Ziele, B-095).

## Regeln und Einschränkungen

Domäne PLAT. Regel „Seiten & Navigation“ aus `CLAUDE.md` (`installPageChrome()`, Eintrag in `src/landing/pages.ts`, `toggleFullscreen()`, `openPage()`/`goHome()`); die Testseite `testing.html` darf für den Einstieg angepasst werden. U2 ist abgeschlossen (`GET /api/level`). Die Abnahme am TV macht nur 🧑.

## Beispiele

Seed `test`, Biom `forest` → 22 Abschnitte, zwei Portale, ein Ausgang.

## Ausnahme- und Fehlerfälle

Server nicht erreichbar → Hinweistext statt leerer Seite (B-092 › Ausnahme- und Fehlerfälle).

## Akzeptanzkriterien

- **AC-01** Seite eingetragen, `installPageChrome()` vorhanden, Projektregeln grün (B-092/AC-01).
- **AC-02** Abbildungs-Funktion getestet (B-092/AC-02).
- **AC-03** Das Golden-Level ergibt im Zeichenmodell genau seine Abschnitte und Objekte (B-092/AC-03).
- **AC-04** Hinweis ohne Server (B-092/AC-04).
- **AC-05** 🧑 hat die Seite mit Controller am TV bedient (B-092/AC-05).
- **AC-06** Die Testseite `testing.html` führt über eine Kachel „Level-Betrachter“ zu `leveltest.html`, Test grün (B-092/AC-06).
- **AC-07** Start-URL-Funktion nur für Wald und Seeds im Namensformat, getestet (B-092/AC-07).

## Offene Fragen

keine. Entschieden 2026-10-02 durch 🧑 (Chat): „Im Spiel starten“ darf einen gleichnamigen Spielstand ersetzen (die Sicherung bleibt).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| U3.1 | `U3.1-zeichenmodell.md` | Umsetzung | autonom | fertig |
| U3.2 | `U3.2-seite.md` | Umsetzung | autonom | fertig |
| U3.3 | `U3.3-testseite.md` | Umsetzung | autonom | offen |
| U3.4 | `U3.4-review.md` | Review | autonom | offen |

## Abnahme

–
