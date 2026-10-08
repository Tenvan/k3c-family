# M4 · SRV · k3c-dev IV: Oberfläche mit Dienste- und Logs-Seite

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** SRV
- **Reife:** bereit
- **Tickets:** B-064, B-068
- **Start-Commit:** 2795d18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (M4 Revision 1 mit B-064, B-068, Wails v2.16.0, React 19.3.0, Radix Themes 3.3.0, plugin-react 6.1.1, Ausnahmen und 5 Sessions)

## Ausgangslage

Nach M1 bis M3 läuft `k3c-dev` ohne Fenster im Terminal; Dienste, Läufe und Logs erreichen nur Agenten über MCP.

## Ziel

`k3c-dev` ist ein Windows-Programm mit Fenster, das den MCP-Server hostet, auf der Dienste-Seite die Dienste führt und auf der
Logs-Seite Dienste, Läufe und JSON-Logs live zeigt. Am Ende sichtbar: `k3c-dev.exe` startet, ein Agent ruft `check_run`, der Lauf erscheint in der
Quellenleiste und seine Ausgabe läuft in der Konsole mit.

## Beteiligte und Zielgruppen

Entwickler am Entwickler-PC; 🧑 gibt Abhängigkeiten und Ausnahmen frei und nimmt die Oberfläche ab.

## Anforderungen

B-064 › Anforderungen, B-068 › Anforderungen.

## Nicht-Ziele

B-064 › Nicht-Ziele, B-068 › Nicht-Ziele. Die MCP-Seite folgt in M5.

## Regeln und Einschränkungen

B-064 und B-068 › Regeln und Einschränkungen. Einschiebbar nach M3. Mit der Freigabe zu genehmigen:

1. **Abhängigkeiten** (Versionen geprüft 2026-09-30): im Modul `tools/k3c-dev` `github.com/wailsapp/wails/v2`
   **v2.16.0** samt Wails-CLI v2.16.0; im Frontend `tools/k3c-dev/frontend/` `react` und `react-dom` **19.3.0**,
   `@radix-ui/themes` **3.3.0**, `@vitejs/plugin-react` **6.1.1** (verlangt Vite 8), `@types/react` und
   `@types/react-dom` **19.3.0**; Vite (`^8.3.1`) und TypeScript (`^7.0.2`) wie im Hauptprojekt. Die Frontend-Tests
   laufen im Vitest des Hauptprojekts, das Frontend bekommt kein eigenes.
2. **Ausnahmen außerhalb der Domäne** (INF, nur in M4.1): `package.json` (`check:dev` baut zuerst das Frontend samt
   Typecheck), `.github/workflows/ci.yml` (Job `k3c-dev` baut zusätzlich Frontend und `wails build`), `.oxlintrc.json`
   (`tools/k3c-dev/frontend/dist` und `wailsjs` ignorieren), `requirements.md` (Wails-CLI, WebView2-Laufzeit),
   `tests/projectRules.test.ts` und `tests/nesting_test.go` (`node_modules` unter `tools/` auslassen, Hinweis aus dem
   Review M1.4).
3. **Fünf Sessions** statt 2–4: Die Logs-Seite (geschätzt ~1000 Zeilen) ist auf M4.3 und M4.4 verteilt
   (Entscheidung 🧑, 2026-09-30, Chat).

## Beispiele

B-064 › Beispiele, B-068 › Beispiele.

## Ausnahme- und Fehlerfälle

B-064 und B-068 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Fenster, Einzelinstanz, gemerkte Größe, CI-Build (B-064/AC-01).
- **AC-02** Kopfzeile, Farbmodus nach der Landingpage und Mock (B-064/AC-02).
- **AC-03** Dienste-Seite mit Karten, Sammelaktionen und Bestätigung (B-068/AC-01, B-068/AC-02, B-068/AC-03).
- **AC-04** Quellenleiste und Konsole live, Dienste eingeschlossen (B-064/AC-03).
- **AC-05** Reiter `Log` und `Fehler (verdichtet)` (B-064/AC-04).
- **AC-06** `check:dev` und CI grün, Grenzen eingehalten (B-064/AC-05).
- **AC-07** Abnahme durch 🧑: Oberfläche im Fenster gesehen, Beispiele aus B-064 und B-068 nachgestellt (Beobachtung).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| M4.1 | `M4.1-geruest.md` | Umsetzung | autonom | fertig |
| M4.2 | `M4.2-dienste-seite.md` | Umsetzung | autonom | fertig |
| M4.3 | `M4.3-quellen-konsole.md` | Umsetzung | autonom | fertig |
| M4.4 | `M4.4-log-fehler.md` | Umsetzung | autonom | fertig |
| M4.5 | `M4.5-review.md` | Review | autonom | fertig |

M4.2 und M4.3 hängen nur von M4.1 ab; M4.4 folgt auf M4.3. Die Review-Session braucht die Abnahme des Fensters
durch 🧑 (AC-07).

## Abnahme

- 2026-09-30, leichtes Review über `2795d18..main` durch drei Reviewer (Sonnet): Wails-App/Go, Dienste-Seite,
  Logs-Seite. Kriterien: AC-01, AC-02, AC-06 (M4.1), AC-03 (M4.2), AC-04 (M4.3), AC-05 (M4.4), AC-07 von 🧑 (M4.5).
- Behoben (schwer): Bindings vor Ende von `startup` griffen auf nil zu (Wails ruft OnStartup in einer Goroutine);
  Beenden wartete bis 60 s auf einen laufenden Start; Konsole zeigte nach Dienst-Neustart alte Zeilen und verlor
  verworfene letzte Zeilen eines Laufs; überlappendes Laden der Konsole warf.
- Behoben (gering): veraltete Antworten in Log/Fehler, doppelte Zeilenschlüssel, Startzeit und Reihenfolge der
  Lauf-Meldungen, Fenstermaße beim Schließen im minimierten Zustand. Neue Tickets: keine.
