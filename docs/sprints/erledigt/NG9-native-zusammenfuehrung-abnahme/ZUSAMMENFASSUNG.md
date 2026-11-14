# NG9 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Natives Spiel: Zusammenführung, Messung und Abnahme
- **Projekt:** NAT

## Ziel

Das native Spiel läuft vom Start bis zum Spielstand durch, ist gemessen, als Paket gebaut und von 🧑 am PC abgenommen; der Mini-PC-Termin ist vorbereitet. Am Ende sichtbar: `task native:game:e2e` grün, Messbericht unter `reports/`, Paket unter `.work/`, Abnahme in den Notizen von B-802.

## Ergebnis

2026-11-11 Review (Claude, `k3c-reviewer`): AC laut Ergebnis der Sessions; Hardware entkoppelt. Neue Tickets: B-806 bis B-816. Abnahme-Ticket des Projekts: B-817. 2026-11-12 NG9.4 Abnahme am PC zu zweit bestanden (🧑). 2026-11-13 NG9.5 (Mini-PC) entfällt (Dialog Ralf), B-786 verworfen; Sprint erledigt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Ende-zu-Ende-Test ohne Fenster grün (`B-802/AC-01`). |
| AC-02 | verschoben | Funktionsabgleich mit `src/scenes/`, Lücken als Tickets (`B-802/AC-02`). |
| AC-03 | geprüft | Messbericht Entwicklungs-PC und Paket für den Mini-PC (`B-802/AC-03`). |
| AC-04 | – | Abnahme 🧑 am PC zu zweit (`B-802/AC-04`). |
| AC-05 | – | Sichtprüfung und Messung am Mini-PC mit Entscheidung (`B-786/AC-01`, Mini-PC-Teil von `B-7 … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-802 | erledigt | AC-01 geprüft, AC-02 verschoben, AC-03 geprüft, AC-04 – | Das native Spiel läuft von Start bis Spielstand durch, ist gemessen und am PC ab … |
| B-786 | verworfen | AC-05 – | Rig-Monarch wird am Mini-PC geprüft und gemessen, sobald das Spiel am PC fertig … |
| B-775 | eingeplant | – | K3C läuft nativ in Go und Ebitengine auf dem Mini-PC, lokal für 1–2 Spieler ohne … |
