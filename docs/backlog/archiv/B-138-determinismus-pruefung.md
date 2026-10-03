# B-138 · Determinismus der Simulation wird gegen Map-Reihenfolge und langsame Ticks geprüft

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** F2
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint F2

## Ausgangslage

Go iteriert über Maps in zufälliger Reihenfolge; schleicht sich ein `range` über eine Map in `engine/sim` oder `engine/level` ein, ändert sich das Ergebnis von Lauf zu Lauf, ohne dass ein Test es sicher bemerkt. Das Ergebnis einer Gleitkomma-Rechnung kann sich zwischen amd64 und arm64 unterscheiden (B-071). Der Takt eines Raums ist ein `time.Ticker` (`engine/room/run.go`); ist ein Tick zu langsam, meldet `engine/room/logging.go` nur eine Warnung, das Verhalten der Simulation dabei ist nicht getestet. Ziel des Plans: auf dem Pi verlangsamen statt überspringen.

## Ziel

Ein Test oder Lint schlägt an, wenn Spielzustand von der Reihenfolge einer Map-Iteration abhängt, und ein Test belegt, dass ein langsamer Takt dieselbe Welt ergibt wie ein schneller. Nutzen: Reproduzierbare Läufe auf PC und Pi; Voraussetzung für Replays (B-159) und Balancing.

## Beteiligte und Zielgruppen

Entwickler und Agenten; Spieler am Pi (sonst andere Abläufe als auf dem PC).

## Anforderungen

- Ein automatischer Check findet `range` über Maps in `engine/sim` und `engine/level`, die den Weltzustand beeinflussen; erlaubte Ausnahmen stehen mit Begründung in einer Liste im Check.
- Ein Test lässt dieselbe Welt (feste Seeds) einmal mit künstlich verlangsamtem Raum-Takt und einmal ohne laufen und vergleicht den Zustand nach N Ticks (gleicher Hash).
- Die arm64-Prüfung selbst gehört zu B-071; dieses Ticket nennt sie nur als Voraussetzung.

## Nicht-Ziele

Golden auf arm64 in der CI (B-071), Ablauf für Golden-Updates (B-137), Replay-Format (B-159).

## Regeln und Einschränkungen

Domäne INF (Lint-Konfiguration, `tests/`, `.github/`). Ein Test in `engine/room/` ist eine Domänen-Ausnahme, die die Sprint-Spec nennt. Keine neue Abhängigkeit ohne Zustimmung von 🧑; nur `engine/rng`, nie `math/rand`.

## Beispiele

Ein neues `for k := range m` über `map[string]*Troop` in `engine/sim/units.go` → der Check meldet Datei und Zeile.

## Ausnahme- und Fehlerfälle

Eine Map wird nur gelesen oder die Schlüssel werden vorher sortiert (`slices.Sorted`) → kein Befund.

## Akzeptanzkriterien

- **AC-01** Der Check meldet ein absichtlich eingefügtes `range` über eine Map in `engine/sim` (Test mit Testdatei im Check) und ist auf dem aktuellen Code grün; `task check:go` läuft ihn mit.
- **AC-02** Ein Go-Test belegt gleichen Weltzustand nach N = 600 Ticks mit und ohne künstlicher Verzögerung des Raum-Takts (Test grün, N im Test benannt).
- **AC-03** Die Ausnahmeliste des Checks hat zu jedem Eintrag einen Satz Begründung (Sichtprüfung).

## Offene Fragen

Lint (z. B. ein Regelsatz in `golangci-lint`) oder eigener Test: bleibt der Umsetzung überlassen, solange keine neue Abhängigkeit entsteht; sonst entscheidet 🧑.

## Notizen

Aus Plan Lücke 18. Verwandt: B-071.
