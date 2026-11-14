# NG15 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Natives Spiel: Ton-Gerät ohne Absturz
- **Projekt:** NAT

## Ziel

Ein defektes oder verschwindendes Ton-Gerät macht das native Spiel stumm, beendet es aber nicht. Am Ende sichtbar: Test mit nachgestelltem Treiberfehler grün, Log-Zeile, Abnahme 🧑 am PC (NG15.3).

## Ergebnis

2026-11-12 Review (Claude, `k3c-reviewer`, NG15.2): AC-01 belegt im Ergebnis von NG15.1 (Commit 403454629); AC-02 angenommen, Validierung offen in NG15.3 (🧑 Ralf, Gerät entfernen). Behobene Befunde: keine schweren. Vermutet: Ton läuft beim Minimieren weiter, Prüfung in NG15.3. Neue Tickets: keine. Abnahme-Ticket für Sim, Balance und Gen: keins nötig (kein Gen-Feature; Sim gesperrt laut Beschluss 2026-11-11). Sprint bleibt aktiv bis NG15.3.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Nachgestellter Treiberfehler: Spiel läuft weiter, Log-Zeile vorhanden (`B-805/AC-01`). |
| AC-02 | offen | Am PC Gerät während des Spiels entfernt, Spiel läuft weiter (Abnahme 🧑, `B-805/AC-02`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-805 | erledigt | AC-01 –, AC-02 offen | Ein Fehler des Ton-Geräts beendet das native Spiel nicht mehr |
