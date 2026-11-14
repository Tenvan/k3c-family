# S2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Protokoll für Skills, Speichern beim Verlassen, Spielmetrik
- **Projekt:** –

## Ziel

Der Client kann Schlag, Skills, Pool und gültige Aktionen je Spieler über das Protokoll bedienen; der Server speichert beim Verlassen und schreibt je Sitzung einen Spielmetrik-Report. Am Ende sichtbar: neue Felder in `docs/protocol.md` mit Beispielen unter `testdata/protocol/`, Spielstand nach Trennung mitten in der Nacht, ein Report in `reports/`.

## Ergebnis

2026-10-05, Review S2.5 (reviewer-s25, nicht der Umsetzer). AC-01, AC-02: S2.1 (Beruf und Tausch aus B-123/AC-02 verschoben → B-283); AC-03, AC-04: S2.2; AC-05, AC-06: S2.3; AC-07: S2.4 (Ereignisse 10,5 Byte je Tick und Gerät, im Budget Q08). Keine schweren Befunde im Diff (S2.4) und in den Stichproben (S2.1–S2.3); neue Tickets B-283 (Rest B-123), B-284 (Flake im Lasttest-Test). Bestätigung durch 🧑 offen: B-278 und B-279 wurden in S2.4 mit `Spec: rückwirkend` ohne Freigabe erledigt und archiviert. Version: v0.7.0 vorgeschlagen (Minor: Protokoll v4, Speichern beim Verlassen und Spielmetrik-Repo …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `docs/protocol.md` beschreibt die neuen Felder, `testdata/protocol/` hat Beispiele, beide … |
| AC-02 | – | Der Server prüft Eingaben und lehnt ungültige ab; Protokollversion erhöht, ältere Clients … |
| AC-03 | – | Verlassen und Trennen des letzten Geräts speichern; ein Schreibfehler lässt den alten Stan … |
| AC-04 | – | Der Spielstand nennt Zeitpunkt und Tag/Nacht/Stufe, die Liste liefert sie; ein Stand ohne … |
| AC-05 | – | Ein Raumlauf erzeugt einen Spielmetrik-Report mit allen Mindestfeldern, ohne Namen, und be … |
| AC-06 | – | Das Report-Schema ist beschrieben und `task check:go` ist grün (B-150/AC-04). |
| AC-07 | – | Ein Gerät bekommt Level und Zustand jeder Stufe, in der ein lokaler Spieler steht; Beispie … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-123 | erledigt | AC-01 –, AC-02 – | Das Protokoll kennt Schlag, Skills, Pool, Berufe und die gültigen Aktionen je Sp … |
| B-147 | erledigt | AC-03 –, AC-04 – | Der Server speichert beim Verlassen und wenn das letzte Gerät getrennt ist, der … |
| B-150 | erledigt | AC-05 –, AC-06 – | Der Server schreibt je Sitzung einen Spielmetrik-Report nach reports/ |
| B-176 | erledigt | AC-07 – | Das Protokoll liefert Level und Zustand jeder Stufe, in der ein lokaler Spieler … |
