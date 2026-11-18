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

1. **Bitgleich zu den Go-Golden bleiben:** rng, Ganzzahlen und der Level-Aufbau (Level-Generator, RU6).
2. **Sim-Fließkomma: gleiches Verhalten.** Dieselben Ereignisse in derselben Reihenfolge, dieselben Ganzzahl-Werte (Gold,
   Tag, IDs, Zählungen), dieselben Kampagnen-Ergebnisse; Fließkommawerte mit Toleranz 1e-6 relativ, absolut 1e-9 nahe 0.
3. **Determinismus innerhalb von Rust bleibt Pflicht:** gleicher Seed ergibt bitgleich dasselbe Spiel (Wiedergabe,
   Spielstände, Koop).
4. **Optimierungen der Sim sind erlaubt** (Umsortieren, schnellere Datenstrukturen, std-Funktionen statt `k3c-det`),
   solange Toleranz und Ereignis-Abgleich halten.
5. **Nach RU41 schreibt Rust eigene Golden;** die Go-Golden werden dann ersetzt.
6. Bitgleichheit zu Go ist ein Prüfwerkzeug des Ports, keine Anforderung an das Spiel.

## Folgen

- Der Orakel-Satz in 006 gilt nur noch für rng, Ganzzahlen und Level; für die Sim gilt dieser Beschluss. Der Vorschlag
  011 in B-1014 bleibt davon unberührt.
- `docs/standards/rust-port.md` (§ 1, § 5), Sprint RU22 (Abgleich mit Ereignis-Vergleich und Toleranz statt Bit-Vergleich),
  RU6 (bleibt bitgleich) und Gate G3 im Projekt RSU sind angepasst.
- Risiko: Toleranz kann eine echte Regel-Abweichung verdecken; der Ereignis- und Ganzzahl-Abgleich fängt sie, die
  Kampagnen-Ergebnisse sind die Gegenprobe.
