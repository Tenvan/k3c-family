# NG1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Natives Spiel: Fundament mit Modul, Vertrag, Simulation im Prozess und Spielschleife
- **Projekt:** NAT

## Ziel

Das Modul `native/k3c-game` steht mit Vertrag, Simulation im Prozess und Spielschleife, sodass NG2–NG8 parallel je ein Paket füllen können. Am Ende sichtbar: `task native:game` zeigt 1–2 Zellen mit Platzhaltern an den Positionen der echten Simulation und großer FPS-Anzeige; `task check:diff` prüft das Modul.

## Ergebnis

2026-11-11 Review (Claude, `k3c-reviewer`): AC-01 bis AC-11 laut Ergebnis NG1.1 bis NG1.6. Behoben: 11 Review-Befunde am Vertrag (Kamera-Regel, Grafik je Level, Einstellungen, Skill-Befehle und Overlay, Geräte neu zuordnen, `ActionHome`, Geräte im Frame, Rig-Mathematik, Allokationen) und 16 Lint-Befunde. Vertrag `view` gilt ab jetzt als eingefroren (`view/doc.go`). Neue Tickets: B-803. Offen: CI-Job `k3c-game` lief noch nie, Fenster nie gestartet, Schriftdatei fehlt. Kein Abnahme-Ticket (keine Sim-, Balance-, Gen-Läufe betroffen). Version: keine vorgeschlagen (noch kein spielbarer Stand).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Modul baut ohne cgo, Ebitengine gepinnt, `replace` auf `k3c` und `k3c-rig` (`B-792/AC-01`) … |
| AC-02 | – | Vertrag `view` mit Platzhaltern je Fachpaket und Test der Importregel (`B-792/AC-02`). |
| AC-03 | – | Simulation im Prozess: gleicher Welt-Hash wie `engine/room` nach 300 Takten mit 2 Spielern … |
| AC-04 | – | Speichern, Liste, Laden, Löschen gegen Temp-Ordner (`B-793/AC-02`). |
| AC-05 | geprüft | Nichts unter `engine/` und `testdata/golden/` geändert (`B-793/AC-03`). |
| AC-06 | – | Grafik-Bibliothek `art` mit Sheets je Level, Rig-Bibliothek, Schrift, Meldung bei fehlende … |
| AC-07 | – | Fenster mit 1–2 Zellen, Platzhaltern und FPS-Anzeige, Lage offline getestet (`B-792/AC-04` … |
| AC-08 | geprüft | `check:diff` prüft das Modul (`B-794/AC-01`). |
| AC-09 | – | CI-Schritt unter Windows (`B-794/AC-02`). |
| AC-10 | – | Die Begriffe „Natives Spiel“, „Frame“ (nativ), „Zelle“ und „Zeichner“ stehen im Glossar (l … |
| AC-11 | – | Aufnahme-Modus mit gesteuerter Szene und Bildfolge samt Streifen-PNG, gleiche Schalter erg … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-792 | erledigt | AC-01 –, AC-02 –, AC-06 –, AC-07 –, AC-11 – | Das native Spiel hat ein eigenes Go-Modul mit Spielschleife, Grafik-Bibliothek u … |
| B-793 | erledigt | AC-03 –, AC-04 –, AC-05 geprüft | Das native Spiel rechnet mit der Go-Simulation im selben Prozess und speichert S … |
| B-794 | erledigt | AC-08 geprüft, AC-09 – | check:diff und CI prüfen das Modul des nativen Spiels |
