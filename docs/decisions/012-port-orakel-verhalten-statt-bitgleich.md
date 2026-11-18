# 012 · Port-Orakel: gleiches Verhalten statt bitgleich (Sim-Fließkomma)

- **Status:** angenommen (Ralf, Dialog im Chat am 2026-11-18, „ja, mach das so“)
- **Datum:** 2026-11-18
- **Projekt:** RSU · Tickets: B-1014 (Vorschlag 011), B-1018 (Port-Leitfaden)
- **Vorgänger:** 006 (Rust-Engine; Orakel-Satz: Rust erzeugt die Golden bitgleich), 008 (natives Spiel in Go)

## Anlass

Die Rust-Umstellung (Projekt RSU) nutzt die Go-Golden als Orakel und verlangte bisher überall Bitgleichheit (006,
`docs/standards/rust-port.md`). Für Sim-Fließkomma zwingt das zu `k3c-det` statt `powf`/`exp`, zu Gos Rechenreihenfolge
und verbietet Optimierungen. Für das Spiel selbst ist die Bit-Identität zu Go ohne Wert; nach RU41 (Go-Engine entfernt)
gibt es Go ohnehin nicht mehr.

## Entscheidung

Nachgeschärft von Ralf am 2026-11-18: Die alten Sim- und Balancing-Ergebnisse spielen keine Rolle mehr; Balancing und Sim-Läufe werden nach der Umsetzung komplett neu gemacht.

1. **rng: bitgleich** zu den Go-Golden.
2. **Level-Generator (RU6): gleiches Ergebnis.** Gleiche Chunks und Objekte (Art, Anzahl, Reihenfolge); Fließkomma mit Toleranz 1e-6 relativ (absolut 1e-9 nahe 0).
3. **Sim (RU22, G3): gleiches Verhalten.** Gleiche Ereignisse in gleicher Reihenfolge, gleiche Ganzzahl-Werte (Gold, Tag, IDs, Zählungen), gleiche Kampagnen-Ergebnisse; Fließkomma mit grober Toleranz, Standard 1e-4 relativ (absolut 1e-9 nahe 0). Zweck: nur Port-Fehler finden.
4. **Keine Pflicht mehr** zur Go-Rechenreihenfolge, zu `k3c-det`-Mathematik und zu Go-Datenstrukturen (lineare Suche, Maps). Idiomatisches, performantes Rust ist ausdrücklich erwünscht; std-Funktionen statt `k3c-det`, Umsortieren und schnellere Datenstrukturen sind erlaubt.
5. **Verbindlich bleiben** die Reihenfolge der Ereignisse und der rng-Aufrufe (Determinismus und Ereignis-Abgleich hängen daran) sowie der Determinismus innerhalb von Rust: gleicher Seed ergibt bitgleich dasselbe Spiel (Wiedergabe, Spielstände, Koop).
6. **Nach RU41 schreibt Rust eigene Golden;** die Go-Golden werden dann ersetzt. Bitgleichheit zu Go ist ein Prüfwerkzeug des Ports, keine Anforderung an das Spiel.

## Folgen

- Der Orakel-Satz in 006 gilt nur noch für den rng; für Level und Sim gilt dieser Beschluss. Der Vorschlag
  011 in B-1014 bleibt davon unberührt.
- `docs/standards/rust-port.md` (§ 1, § 5), Sprint RU22 (Ereignis-Vergleich, Toleranz 1e-4), RU6 (gleiches Ergebnis, Toleranz 1e-6), die Sim-Sprints RU10, RU11, RU14, RU15, RU18, RU19 (Regeln) und Gate G3 im Projekt RSU sind angepasst.
- Risiko: Toleranz kann eine echte Regel-Abweichung verdecken; der Ereignis- und Ganzzahl-Abgleich fängt sie, die
  Kampagnen-Ergebnisse sind die Gegenprobe.
