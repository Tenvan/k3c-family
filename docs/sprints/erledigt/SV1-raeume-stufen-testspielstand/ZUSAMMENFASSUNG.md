# SV1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Raum mit allen Stufen, Voll-Ausbau-Spielstand, leere Test-Räume
- **Projekt:** WRT

## Ziel

Jeder Raum spielt die ganze Insel, und ein Klick im Level-Betrachter startet einen Spielstand mit allen Gebäuden voll ausgebaut. Am Ende sichtbar: Neuer Raum mit allen fünf Stufen, Level-Betrachter startet einen voll ausgebauten Spielstand.

## Ergebnis

- 2026-10-14, SV1.5: AC-01 (SV1.3, SV1.4), AC-02 und AC-04 (SV1.1), AC-03 (SV1.1 Server, SV1.2 Client), AC-05 (SV1.3), AC-06 (alle; `task check` und `task check:go` grün) mit Nachweis. Die Testseite braucht leere Test-Räume nicht länger, AC-04 ist also erfüllt. - Behoben im Review: Die Dev-API nahm Seeds an, deren Name `voll-<seed>` `store.Saves` ablehnt (Großbuchstaben, `_`, über 32 Zeichen); das ergab 500 statt 400. Jetzt gilt `^[a-z0-9-]{1,27}$` wie im Client, mit Test. - Offen für 🧑: B-315/AC-03, also Klick auf „Voll ausgebaut starten“ in `leveltest.html` am PC beobachten. Bis dahin bleibt …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Der Level-Betrachter erzeugt einen Spielstand mit allen Gebäuden voll ausgebaut (B-315/AC- … |
| AC-02 | – | Ein neuer Raum legt die Insel mit allen Stufen an, für die es ein Biom gibt (B-199/AC-01, … |
| AC-03 | – | Der Raum erzeugt alle fünf Stufen und der Client kennt Eisenstollen und Kristallhöhle (B-2 … |
| AC-04 | – | Leere Test-Räume schließen sofort statt nach der Leer-Frist (B-204/AC-01). |
| AC-05 | – | Der Spielstand „voll ausgebaut“ ist nur im Dev-Mode erzeugbar; ohne Dev-Mode antwortet die … |
| AC-06 | geprüft | `task check` und `task check:go` grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-315 | eingeplant | AC-01 – | Der Level-Betrachter erzeugt einen Spielstand mit allen Gebäuden voll ausgebaut |
| B-199 | erledigt | AC-02 – | Ein neuer Raum legt die Insel mit allen Stufen an, für die es ein Biom gibt |
| B-290 | erledigt | AC-03 – | Der Raum erzeugt alle fünf Stufen und der Client kennt Eisenstollen und Kristall … |
| B-204 | erledigt | AC-04 – | Leere Test-Räume schließen sofort statt nach der Leer-Frist |
