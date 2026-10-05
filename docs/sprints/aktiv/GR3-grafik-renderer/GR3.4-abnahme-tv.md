# GR3.4 · Abnahme am TV

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Branch:** gr3/4-abnahme-tv
- **Abhängig von:** GR3.3
- **Tickets:** B-010
- **Kriterien:** AC-04, AC-06

## Ziel

🧑 hat den Renderer auf der Xbox am TV mit zwei Spielern im Split-Screen angesehen: Hub-Stufen und Mauer-/Turm-Materialstufen sind unterscheidbar, Parallax je Biom ohne Darstellungsfehler. Das Ergebnis steht in dieser Datei.

## Kontext

Ein Agent nimmt diese Session nicht und bereitet sie nicht vor. Dauer etwa 15 Minuten. Hardware-Session nach `docs/arbeitsweise.md` › Hardware entkoppelt: keine Abhängigkeit des Reviews GR3.3. Bis zur Abnahme gelten AC-04 und AC-06 als `angenommen, Validierung offen (GR3.4)`. Ohne Stufe im Snapshot zeigen Mauer und Turm die Stein-Grafik (bewusste Abweichung aus GR3.3, kein Mangel).

## Erlaubte Dateien

- diese Datei (Ergebnis, Status)
- Sprint-README (nur Tabelle der Sessions und Abnahme), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Code-Änderungen; neue Grafiken (GR2).

## Schritte

1. Stand im Heimnetz bereitstellen (`task serve` oder Pi), `index.html` auf der Xbox öffnen, Spiel mit zwei Controllern starten.
2. Hub ausbauen, Mauer und Turm ansehen, in jedem Biom die Parallax-Ebenen prüfen.
3. Ergebnis eintragen (Datum, Gerät, Befund); Mängel als Tickets; `Status: fertig` (bei Mängeln `blockiert`). Ist es die letzte offene Session, Sprint nach `erledigt/` verschieben.

## Fertig, wenn

- [ ] AC-04: 🧑 bestätigt am TV, dass Hub-Stufen und Materialstufen unterscheidbar sind.
- [ ] AC-06: 🧑 sieht mit 2 Spielern im Split-Screen keine Darstellungsfehler.

## Prüfen

Manuell durch 🧑 an der Xbox.

## Ergebnis

–
