# M5 · SRV · k3c-dev V: MCP-Seite mit Monitoren und Statistik

- **Status:** aktiv
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-065
- **Start-Commit:** 3b6ef83
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (Pauschalauftrag „beide komplett autonom fertig stellen“: M5 Revision 1 mit B-065, 5 Sessions)

## Ausgangslage

Nach M4 hat `k3c-dev` ein Fenster mit Logs-Seite; der Reiter `MCP` zeigt nur einen Hinweis.

## Ziel

Die MCP-Seite zeigt Server, Verbindungen, Tools, Live-Monitore, Aufruf-Log und Statistik. Am Ende sichtbar: Zwei
parallele Agenten-Aufrufe erscheinen als zwei Spuren im Aufruf-Log, die Statistik zeigt p95 und Ausreißer je Tool.

## Beteiligte und Zielgruppen

Entwickler, die Agenten-Arbeit beobachten; 🧑 nimmt die Seite ab.

## Anforderungen

B-065 › Anforderungen.

## Nicht-Ziele

B-065 › Nicht-Ziele.

## Regeln und Einschränkungen

B-065 › Regeln und Einschränkungen. Einschiebbar nach M4. Keine neuen Abhängigkeiten, keine Ausnahmen außerhalb der Domäne.
**Fünf Sessions** statt 2–4: Live-Monitore und Aufruf-Log sind je eine eigene Session (wie M4; Pauschalauftrag 🧑).

## Beispiele

B-065 › Beispiele.

## Ausnahme- und Fehlerfälle

B-065 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Übersicht mit Server, Verbindungen, Instructions und Neustart (B-065/AC-01).
- **AC-02** Tool-Kacheln und Live-Monitore (B-065/AC-02).
- **AC-03** Aufruf-Log mit Graph-Spuren und Filtern (B-065/AC-03).
- **AC-04** Statistik für Sitzung und Gesamtzeit (B-065/AC-04).
- **AC-05** `check:dev` und CI grün, Grenzen eingehalten (B-065/AC-05).
- **AC-06** Abnahme durch 🧑: Seite im Fenster gesehen, Beispiele aus B-065 nachgestellt (Beobachtung).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| M5.1 | `M5.1-uebersicht.md` | Umsetzung | autonom | fertig |
| M5.2 | `M5.2-live-monitore.md` | Umsetzung | autonom | fertig |
| M5.3 | `M5.3-aufruf-log.md` | Umsetzung | autonom | offen |
| M5.4 | `M5.4-statistik.md` | Umsetzung | autonom | offen |
| M5.5 | `M5.5-review.md` | Review | autonom | offen |

M5.2 bis M5.4 hängen nur von M5.1 ab. Die Review-Session braucht die Abnahme der Seite durch 🧑 (AC-06).

## Abnahme

–
