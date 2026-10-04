# B-206 · Die Planungsseite von k3c-dev ist eine React-Ansicht aus denselben Daten wie die MCP-Tools

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** M8
- **Erstellt:** 2026-10-04
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 2 (Ergänzung Glossar), durch 🧑; umfasst B-205, B-206, B-207

## Ausgangslage

„Sprints & Backlog“ ist eine eigenständige HTML-Seite (`tools/k3c-dev/internal/planning/page.html`, 257 Zeilen mit eigenem CSS und Script), die Go per `page.go` mit JSON füllt und das Frontend im iframe zeigt (`frontend/src/planning/PlanningPage.tsx`). Sie hat ein eigenes Theme, eigenen Zustand im `sessionStorage` und lässt sich nicht mit Radix-Komponenten oder den übrigen Seiten teilen.

## Ziel

Die Seite rendert React aus `planning.Data`, denselben Daten, die `plan_list` liefert. `page.html` und `page.go` entfallen.

## Beteiligte und Zielgruppen

🧑 und Entwickler, die die Planung in k3c-dev ansehen.

## Anforderungen

- Binding `PlanningData()` liefert `planning.Data` als JSON; `PlanningPage()`, `page.go`, `page.html` und der `?raw`-Import im Mock entfallen.
- React-Ansicht mit dem Umfang von heute: Suche, Filter (Domäne, Status, Typ), Sprint-Karten mit Fortschritt, Session-Liste mit nächster offener Session, Worktree-Markierung, Backlog gruppiert, Detail-Panel mit Markdown (`ui/MarkdownView.tsx`).
- Theme und Bausteine wie die übrigen Seiten (Radix Themes, `styles/planning.css`), Filter und Auswahl über `lib/prefs.ts`.
- Aktualisierung bei `planning:changed` ohne Verlust von Filter und Auswahl.
- Mock (`api/mockPlanning.ts`) liefert `PlanningData` direkt.
- Liegt `docs/glossar.md` vor, erscheint sie als weitere Ansicht „Glossar“ neben Plan und Fragenkatalog; fehlt die Datei, fehlt auch der Umschalter (kein Fehler).

## Nicht-Ziele

Bearbeiten in der Oberfläche (ändert nur MCP, B-205); GitHub-Status (B-207); neue Abhängigkeit (Radix und React reichen).

## Regeln und Einschränkungen

Domäne SRV, nur `tools/k3c-dev/`. Datei ≤ 400, Funktion ≤ 60 Zeilen; reine Logik (Filter, Gruppierung, nächste Session) in einer `.ts`-Datei mit Vitest.

## Beispiele

Filter „SRV“ + Suche „mcp“ → nur passende Sprints und Tickets; Klick auf `M8.1` → Session-Text im Detail-Panel; `docs/glossar.md` angelegt → Umschalter „Glossar“ erscheint nach `planning:changed`; Datei in `docs/` geändert → Liste neu, Auswahl bleibt.

## Ausnahme- und Fehlerfälle

`docs/` nicht lesbar → `NoticeCard` mit Fehler wie heute; ausgewählte ID verschwindet nach Änderung → Detail-Panel leer, kein Absturz.

## Akzeptanzkriterien

- **AC-01** `page.html`, `page.go` und das Binding `PlanningPage` sind gelöscht; `grep -r "page.html" tools/k3c-dev` findet nichts.
- **AC-02** Die React-Ansicht zeigt im Mock Sprints, Sessions, Backlog, Filter und Detail-Panel; Vitest deckt Filter, Gruppierung und nächste Session ab.
- **AC-03** Nach `planning:changed` lädt die Ansicht neu und behält Filter und Auswahl (Test oder Mock-Ereignis).
- **AC-04** Mit `docs/glossar.md` zeigt die Planung die Ansicht „Glossar“, ohne die Datei keinen Umschalter; Go-Test für beide Fälle, Mock mit Glossar.

## Offene Fragen

keine

## Notizen

–
