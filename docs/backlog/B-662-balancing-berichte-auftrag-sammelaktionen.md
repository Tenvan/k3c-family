# B-662 · Die Workbench liefert Auftrag und Label je Bericht und verwaltet Berichte gesammelt

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Berichtsliste der Balancing-Seite kommt aus `tools/k3c-workbench/internal/simrun/reports.go` › `ReportInfo`: Pfad, Ordner des Laufs (`simtest-run-1` …), Änderungszeit, Seeds, Pass, Varianten, Art, Bewertung. Label und Auftrag (Bot, Spieler, Welt, Tiefe, Tage, Grad, Hilfen, Baseline) fehlen; die Seite kann Berichte deshalb nicht unterscheiden und nicht erneut starten. Der Auftrag steht nur am Lauf (`simrun/info.go` › `Info.Spec`), und die Läufe kennt die Seite nur für die laufende Sitzung. Ob die Berichts-JSON den Auftrag heute schon enthält, ist ungeprüft.

Bewerten und Löschen gehen je Bericht (`BalanceReportRate`, `BalanceReportDelete`, B-612). Eine Baseline sichert nur das MCP-Tool `sim_test {action: baseline}` (`internal/mcpsrv/simtest_baseline.go`). Während ein Lauf läuft, fragt die Seite alle 1,5 s `balanceRuns` ab (`useBalancing.ts` › `POLL_MS`), obwohl die Workbench einen Ereignis-Weg hat (`uiapi_events.go`: Wails-Ereignisse und SSE).

## Ziel

Die Go-Seite liefert alles, was die neue Liste (B-660) und der Lauf-Editor (B-661) brauchen: erkennbare Berichte mit Auftrag, Sammel-Aktionen, Baseline aus der Oberfläche, Ereignisse statt Abfrage.

## Beteiligte und Zielgruppen

Ralf beim Balancing; Agent DEV.

## Anforderungen

- Jeder neue Bericht speichert Label und Auftrag (`simrun.Spec`) und die Lauf-ID; `ReportInfo` liefert daraus `label`, `spec` und `getroffene/alle Ziele`. Alte Berichte ohne Auftrag liefern diese Felder leer.
- Sammel-Aufrufe: mehrere Berichte bewerten und löschen in einem Aufruf (gleiche Prüfungen wie B-612: nur Pfade unter `reports/`, JSON und Markdown, Bewertung fällt weg, leerer Lauf-Ordner verschwindet).
- „Als Baseline sichern“ für einen fertigen, durchgelaufenen Bericht aus der Oberfläche, mit derselben Logik wie `sim_test {action: baseline}` (ein Weg, kein zweiter Code).
- Ereignisse über den bestehenden Bus (`uiapi_events.go`): Lauf geändert (Phase, Fortschritt, Zustand) und Berichte geändert; die Seite lädt nur noch bei Ereignis und beim Seitenwechsel.
- Werte aller variierbaren Dateien in einem Aufruf (Datei, Pfad, Wert) für die Suche des Daten-Browsers (B-661).
- Option im Auftrag: jede feste Variante als eigener Lauf gegen denselben Basis-Lauf (n + 1 Läufe in der Warteschlange, gemeinsames Label-Präfix).

## Nicht-Ziele

- Oberfläche (B-659, B-660, B-661).
- Änderung des Formats der Varianten (`varyArg`) oder der Berichts-Kennzahlen.
- Lauf-IDs über Neustarts eindeutig halten: B-616.
- Nachträgliches Ergänzen des Auftrags in alten Berichten.

## Regeln und Einschränkungen

- Workbench-Code liegt in `tools/k3c-workbench`; `engine/` importiert nichts aus `tools/`.
- Gleiche Schnittstelle für Wails-Bindings und HTTP-Client (`api/wails.ts`, `api/http.ts`, Mock `api/mockBalance.ts`).
- `docs/standards/workbench-seiten.md` › Keine Intervalle.
- Logging mit Emojis (`CLAUDE.md` › Regeln).
- Komplexitäts-Budget: Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

- Lauf „Insel 1 solo · ohne Hilfen“ endet → Ereignis „Berichte geändert“ → Liste zeigt den Bericht mit Label, Bot `wirtschaft`, 1 Spieler, Welt `standard`, 40 Tage, `3/16 Ziele`.
- Drei Berichte gewählt → Löschen → ein Aufruf, drei Berichte samt Markdown und Bewertung weg.
- Bericht M1 durchgelaufen → „Als Baseline sichern“ → `reports/baseline/M1/` wie mit `sim_test`.
- Auftrag mit drei festen Varianten und Option „je Variante“ → vier Läufe in der Warteschlange: Basis und je Variante einer.

## Ausnahme- und Fehlerfälle

- Pfad außerhalb `reports/` in einem Sammel-Aufruf → ganzer Aufruf abgelehnt, nichts gelöscht.
- Baseline sichern für einen nicht durchgelaufenen Bericht oder einen vorhandenen Namen → Fehler mit Grund (wie `sim_test`).
- SSE-Strom bricht ab → die Oberfläche lädt beim Wiederverbinden die Listen einmal neu.
- „Je Variante“ mit Bereichen (Abstimmung) → abgelehnt mit Hinweis.

## Akzeptanzkriterien

- **AC-01** Go-Test: Ein Lauf schreibt Label, Auftrag und Lauf-ID in den Bericht; `Reports` liefert sie und `x/y Ziele`; ein alter Bericht ohne Auftrag liefert leere Felder ohne Fehler.
- **AC-02** Go-Test: Sammel-Bewerten und Sammel-Löschen; ein fremder Pfad lehnt den ganzen Aufruf ab.
- **AC-03** Go-Test: „Als Baseline sichern“ aus der Oberfläche und `sim_test {action: baseline}` nutzen dieselbe Funktion und erzeugen dieselben Dateien.
- **AC-04** Go-Test: Lauf- und Berichts-Änderungen erzeugen Ereignisse auf dem Bus; Vitest: `useBalancing` lädt auf Ereignis, ohne Intervall.
- **AC-05** Go-Test: Werte aller variierbaren Dateien in einem Aufruf; Option „je Variante“ legt n + 1 Läufe an und lehnt Bereiche ab.

## Offene Fragen

- Wo steht der Auftrag: in der Berichts-JSON selbst oder in einer Nebendatei im Lauf-Ordner? Default (Claude): in der Berichts-JSON, Feld `spec`. Entscheidet 🧑.
- „Je Variante“: ein gemeinsamer Basis-Lauf oder die gewählte Baseline als Basis ohne neuen Lauf? Default (Claude): neuer Basis-Lauf mit gleichen Seeds. Entscheidet 🧑.

## Notizen

- Befund: Design-Canvas „Balancing-Seite Mock“ (2026-10-29), Board „Befund: Ist-Zustand“.
- Voraussetzung für die volle Zeile in B-660 und die Option „je Variante“ in B-661.
