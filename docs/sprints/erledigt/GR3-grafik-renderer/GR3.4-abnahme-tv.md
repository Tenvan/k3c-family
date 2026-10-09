# GR3.4 · Abnahme am TV

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** gr3/4-abnahme-tv
- **Abhängig von:** GR3.3
- **Tickets:** B-010
- **Kriterien:** AC-04, AC-06

## Ziel

🧑 hat den Renderer auf der Xbox am TV mit zwei Spielern im Split-Screen angesehen: Hub-Stufen und Mauer-/Turm-Materialstufen sind unterscheidbar, Parallax je Biom ohne Darstellungsfehler. Das Ergebnis steht in dieser Datei.

## Kontext

**Controller zurückgestellt (B-314, 2026-10-06):** 🧑 prüft vorerst nur am PC mit Tastatur und Maus (1 Spieler; 2 Spieler an einer Tastatur erst mit B-316). Controller-, Vibrations- und Xbox-Schritte dieser Session sind nach B-314 verschoben; die Kriterien bleiben, ihr Controller-Anteil gilt als `angenommen, Validierung offen (B-314)`.

**Spielstand (B-315):** Für die Sicht auf alle Gebäude in allen Ausbaustufen braucht 🧑 einen voll ausgebauten Spielstand aus dem Level-Betrachter (B-315); bis dahin ist nur Stufe 1 sichtbar (Protokoll liefert die Stufen erst mit B-208).

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

2026-10-06, **Tastatur und Maus (Sammelaussage):** 🧑 (Ralf) im Chat: „Alle Tastatur und Maussteuerungen liefen bisher wie definiert.“ Gilt für den Tastatur- und Maus-Anteil dieser Session am PC; Darstellung, Ton und Controller (B-314) sind damit nicht abgenommen, die Session bleibt `offen`.


2026-10-07, **Nicht abgenommen, verworfen auf Entscheidung 🧑 (Ralf, Chat).** `Status: fertig` nur, weil die Session-Vorlage kein `verworfen` kennt (B-338). Bis zur Umsetzung von B-337 (HUD-Elemente ohne Überlagerung) finden keine weiteren Anzeige- und Touch/Tasten-Abnahmen statt; alle bisherigen und offenen werden geschlossen. Die Abnahme dieser Session geht vollständig in die neue Gesamtprüfung **B-337/AC-05** über (Prüfliste dort). Bisherige Nachweise oben bleiben als Vorgeschichte stehen.
