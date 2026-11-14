# SP10 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Diagnose-TUI
- **Projekt:** –

## Ziel

`cmd/k3c-tui` (Bubble Tea) zeigt den laufenden Server und führt die Diagnose-Aktionen aus. Am Ende sichtbar: Räume, Geräte, Tick-Dauer und Log live, auch im Docker-Container (`docker exec -it k3c k3c-tui`).

## Ergebnis

Review 2026-10-01 (SP10.3): `task check`, `task check:go`, `task check:dev` grün, Diff `5c2559a..main` geprüft. Behoben: „Gerät trennen“ meinte nach einem Neuladen der Liste womöglich ein anderes Gerät (jetzt Kennung beim Fragen gemerkt). AC-01: SP10.1 (Anzeige, `-once`, Fehlerfälle, Modell-Takt; Image und CI-Schritt in der Datei geprüft, Lauf in der CI); AC-02: SP10.2 (Raum ansehen, trennen mit Rückfrage, sichern, Log folgen, alle gegen einen Test-Server). Nicht geprüft: Docker-Build und Lauf der CI, interaktiver Lauf gegen einen echten Server. Neue Tickets: keine. B-002 erledigt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `k3c-tui` zeigt die Räume live und `-once` druckt sie; das Image enthält `k3c-tui` und die … |
| AC-02 | – | Die Aktionen Raum ansehen, Gerät trennen, Spielstand sichern und Log folgen wirken gegen e … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-002 | erledigt | AC-01 –, AC-02 – | Diagnose-TUI zeigt den laufenden Server |
