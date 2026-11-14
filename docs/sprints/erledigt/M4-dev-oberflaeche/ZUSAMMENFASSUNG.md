# M4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** k3c-dev IV: Oberfläche mit Dienste- und Logs-Seite
- **Projekt:** –

## Ziel

`k3c-dev` ist ein Windows-Programm mit Fenster, das den MCP-Server hostet, auf der Dienste-Seite die Dienste führt und auf der Logs-Seite Dienste, Läufe und JSON-Logs live zeigt. Am Ende sichtbar: `k3c-dev.exe` startet, ein Agent ruft `check_run`, der Lauf erscheint in der Quellenleiste und seine Ausgabe läuft in der Konsole mit.

## Ergebnis

- 2026-09-30, leichtes Review über `2795d18..main` durch drei Reviewer (Sonnet): Wails-App/Go, Dienste-Seite, Logs-Seite. Kriterien: AC-01, AC-02, AC-06 (M4.1), AC-03 (M4.2), AC-04 (M4.3), AC-05 (M4.4), AC-07 von 🧑 (M4.5). - Behoben (schwer): Bindings vor Ende von `startup` griffen auf nil zu (Wails ruft OnStartup in einer Goroutine); Beenden wartete bis 60 s auf einen laufenden Start; Konsole zeigte nach Dienst-Neustart alte Zeilen und verlor verworfene letzte Zeilen eines Laufs; überlappendes Laden der Konsole warf. - Behoben (gering): veraltete Antworten in Log/Fehler, doppelte Zeilenschlüs …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Fenster, Einzelinstanz, gemerkte Größe, CI-Build (B-064/AC-01). |
| AC-02 | geprüft | Kopfzeile, Farbmodus nach der Landingpage und Mock (B-064/AC-02). |
| AC-03 | geprüft | Dienste-Seite mit Karten, Sammelaktionen und Bestätigung (B-068/AC-01, B-068/AC-02, B-068/ … |
| AC-04 | geprüft | Quellenleiste und Konsole live, Dienste eingeschlossen (B-064/AC-03). |
| AC-05 | geprüft | Reiter `Log` und `Fehler (verdichtet)` (B-064/AC-04). |
| AC-06 | geprüft | `check:dev` und CI grün, Grenzen eingehalten (B-064/AC-05). |
| AC-07 | geprüft | Abnahme durch 🧑: Oberfläche im Fenster gesehen, Beispiele aus B-064 und B-068 nachgestellt … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-064 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-04 geprüft, AC-05 geprüft, AC-06 geprüft, AC-07 geprüft | k3c-dev hat eine Oberfläche mit Logs-Seite für Läufe und JSON-Logs |
| B-068 | erledigt | AC-03 geprüft, AC-07 geprüft | k3c-dev zeigt die Dienste als Karten mit Zustand, Metriken und Log-Level |
