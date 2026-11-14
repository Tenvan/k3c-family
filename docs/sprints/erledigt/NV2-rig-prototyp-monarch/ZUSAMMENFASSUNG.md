# NV2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Rig-Prototyp: Monarch aus Teilen in Ebitengine
- **Projekt:** NAT

## Ziel

Ein Prototyp unter `native/` zeichnet Monarch und Pferd aus Teil-Texturen und dem Rig aus `tools/paper`, kombiniert beide am Sattel und zeigt Waffe und Rüstung als optionale Elemente. Am Ende sichtbar: Fenster mit Rig und Sheet nebeneinander, Differenzbild und ein JSON-Bericht unter `reports/`.

## Ergebnis

2026-11-11, geschlossen auf Anweisung 🧑 (Chat). AC-01 bis AC-05 laut Ergebnis NV2.1 bis NV2.3 (B-778 › Notizen); AC-06 verschoben auf B-786. Der Rig-Weg läuft in NV3 (B-789) weiter. Neue Tickets: B-786. Version: keine vorgeschlagen (Prototyp).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Teile-Export für Monarch und Pferd, Rig-Datei mit Plätzen, byte-gleich bei Wiederholung (` … |
| AC-02 | geprüft | Rig-Rechnung in Go gleich `tools/paper` für Figur und Vierbeiner (`B-778/AC-02`). |
| AC-03 | – | Fenster mit Rig und Sheet, Umhang, Reiter auf Rig-Pferd in drei Gangarten, Waffe und Rüstu … |
| AC-04 | geprüft | Bildvergleich gegen das Sheet mit Zahl und Differenzbild (`B-778/AC-04`). |
| AC-05 | geprüft | Messbericht Entwicklungs-PC mit 2, 50 und 200 Rig-Figuren (`B-778/AC-05`). |
| AC-06 | verschoben | Sichtprüfung und Messwerte Mini-PC (`B-778/AC-06`, 🧑). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-778 | eingeplant | AC-01 geprüft, AC-02 geprüft, AC-03 –, AC-04 geprüft, AC-05 geprüft, AC-06 verschoben | Ein dritter Prototyp setzt den Monarchen in Ebitengine zur Laufzeit aus Rig-Teil … |
