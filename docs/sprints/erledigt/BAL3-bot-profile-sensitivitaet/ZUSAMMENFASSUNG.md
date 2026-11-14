# BAL3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Bot-Profile, Sensitivität und Grad-Kurven
- **Projekt:** –

## Ziel

Der Tester deckt weitere Spielweisen ab und zeigt, welche Kennzahlen bei einer Wertänderung kippen. Am Ende sichtbar: Berichte für vier neue Profile, ein Sensitivitäts-Bericht und eine Kurve je Schwierigkeitsgrad.

## Ergebnis

2026-10-05, Review BAL3.4 (Sonnet, nur Diff): keine schweren Befunde; `task check:go` und `task check` grün. Kriterien: AC-05 in BAL3.1, AC-01 und AC-02 in BAL3.2, AC-03, AC-04 und AC-06 in BAL3.3 nachgewiesen. Neue Tickets: B-300 („Mauern zuerst“ spielt wie „sparsam“, Hub-Ausbau fehlt, Frage an 🧑), B-301 (`--vary` auf nicht neu geladene Datei ohne Fehler). Version: v0.12.0 vorgeschlagen (Minor: neue Bot-Profile, Sensitivitäts-Lauf und Grad-Kurven im Werkzeug; aktuell v0.11.0, gemeinsam mit S6); gesetzt erst nach Bestätigung durch 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Jedes der vier neuen Profile liefert mit gleichem Seed und gleichen Daten byte-gleiche Ken … |
| AC-02 | geprüft | Kein Kind-Bot: Er ist Nicht-Ziel (Q19, später); der Tester enthält kein Fehler-Profil und … |
| AC-03 | – | Ein Sensitivitäts-Lauf (±10 %, ±25 %) erzeugt einen Bericht mit gekippten Kennzahlen (B-15 … |
| AC-04 | geprüft | Der Bericht enthält je Schwierigkeitsgrad eine Kurve über die Tage (B-158/AC-04). |
| AC-05 | geprüft | Profile und Grad-Kurven stehen mit Beschluss von 🧑 in der Dokumentation des Testers (B-158 … |
| AC-06 | geprüft | `task check:dev` ist grün (der Tester liegt unter `tools/k3c-dev`, B-099). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-158 | erledigt | AC-01 –, AC-02 geprüft, AC-03 –, AC-04 geprüft, AC-05 geprüft | Der Tester kennt weitere Bot-Profile, Sensitivitäts-Läufe und Kurven je Schwieri … |
