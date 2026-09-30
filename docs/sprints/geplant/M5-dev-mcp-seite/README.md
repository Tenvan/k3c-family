# M5 · SRV · k3c-dev V: MCP-Seite mit Monitoren und Statistik

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-065
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- M5.1 Übersicht Band 1 und 2 links: Server- und Verbindungskarte, Instructions-Dialog, Neustart, Tool-Kacheln (AC-01, AC-02).
- M5.2 Live-Monitore (Reihen-Berechnung, Säulen, Liniendiagramm) und Aufruf-Log mit Graph-Spuren (AC-02, AC-03).
- M5.3 Statistik: Kennzahlen, sortierbare Tabelle, Aufklappen, Seitenspalte; Mock ergänzen (AC-04, AC-05).
- M5.4 🔍 Review, 🧑 prüft die Seite (AC-06, alle).

## Abnahme

–
