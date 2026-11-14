# GEB18 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Hintergründe der fünf Stufen: vierfache Auflösung, fünf Varianten je Stufe nach Seed
- **Projekt:** GEB

## Ziel

Die fünf Stufen haben detailreiche Hintergründe in vierfacher Auflösung mit fünf Varianten je Stufe, die nach Seed abwechseln. Am Ende sichtbar: Beim Laufen durch eine Stufe wechseln die Hintergründe sichtbar zwischen Varianten, gleicher Seed zeigt auf PC und Xbox dieselbe Folge; das Studio zeigt alle Varianten in der Szene „Umgebung“.

## Ergebnis

2026-10-24, Claude (GEB18.6): `task check` und `task check:go` grün; `task check:race` lokal übersprungen (kein C-Compiler), die CI prüft `-race`. AC-01 bis AC-04 und AC-06 sind laut den Ergebnissen von GEB18.1 bis GEB18.5 erfüllt. Claude hat den Diff gelesen (backdrop.go, objectView/stageView) und keine schweren Befunde gefunden. 2026-10-31: AC-05 von Ralf (🧑) am Gerät abgenommen, im Chat bestätigt; Claude hat die Prüfung nicht selbst gesehen. Alle Kriterien erfüllt, Sprint erledigt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Generator: vierfache Auflösung (Dichte 128) und Varianten (B-618/AC-01). |
| AC-02 | geprüft | Je Stufe mindestens fünf Varianten mit drei Schichten in vierfacher Auflösung, im Katalog … |
| AC-03 | geprüft | Variante je Abschnitt deterministisch nach Seed, Nachbarn verschieden (B-618/AC-03). |
| AC-04 | geprüft | Protokoll-Feld beschrieben, Client zeichnet die gemeldete Variante und lädt nur Benötigtes … |
| AC-05 | – | 🧑 nimmt am Gerät ab, Xbox-Speicher und Ladezeit im Budget (B-618/AC-05). |
| AC-06 | geprüft | Alte Schichten ohne Variante gelöscht, keine Verweise mehr (B-618/AC-06). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-618 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 –, AC-06 geprüft | Die Hintergründe der fünf Stufen sind detailreich (vierfache Auflösung) und wech … |
