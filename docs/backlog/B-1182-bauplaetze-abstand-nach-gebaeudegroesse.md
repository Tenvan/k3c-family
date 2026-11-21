# B-1182 · Die festen Bauplätze im Hub überlappen sich, ihr Abstand soll der Gebäudegröße folgen

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** LA1
- **Projekt:** RSU
- **Erstellt:** 2026-11-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

🧑 2026-11-21 (Nachprüfung LA1.5): Im Hub überlappen sich die Gebäude oft. Die Bauplatz-Positionen stehen fest in den Daten (`data/hub.json`, Platz-Berechnung der Sim) und kennen die Breite der Gebäude nicht; seit den Größenstufen (LA1.1, `data/sizes.json`) und je Welt sind die Gebäude unterschiedlich breit.

## Ziel

Die Plätze im Hub richten sich nach der Breite des Gebäudes (je Welt anders), so dass sich nichts überlappt und die Lücken gleichmäßig sind.

## Beteiligte und Zielgruppen

🧑 entscheidet über Abstände und nimmt am PC ab.

## Anforderungen

- Abstand zweier Nachbar-Plätze = halbe Breite links + halbe Breite rechts + Mindestlücke, aus den Breiten der Gebäude-Assets der Welt.
- Deterministisch, 2+ Spieler, Spielstand-Format prüfen (Positionen in Spielständen).
- Neue Mechanik nur in Rust (`k3c-sim`, Entscheidung 013); die Go-Engine bleibt eingefroren, Golden nicht zugunsten von Rust ändern.

## Nicht-Ziele

Neue Gebäude, Grafik.

## Regeln und Einschränkungen

Sperre golden und spielstand: Positionen der Plätze hängen an Go-Golden und alten Spielständen; vor Umsetzung mit 🧑 klären, ob alte Spielstände neu angeordnet werden.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `data/hub.json`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-level/**`
- **Abhängig von:** LA1.1
- **Braucht 🧑:** Entscheidung | Abnahme
- **Größe:** M
- **Sperre:** golden | spielstand

## Beispiele

Schmiede 4,8 Units und Taverne 6 Units breit: der Abstand der Mittelpunkte ist mindestens 5,4 Units plus Lücke.

## Ausnahme- und Fehlerfälle

Zu wenig Platz im Hub: Reihenfolge und Mauern behalten Vorrang, Meldung im Log.

## Akzeptanzkriterien

- **AC-01** Test: in jeder festen Welt überlappen sich keine zwei gebauten Plätze (Breiten aus den Assets).
- **AC-02** Abnahme 🧑 am PC.

## Offene Fragen

🧑: Alte Spielstände neu anordnen oder nur neue Spiele?

## Notizen

Herkunft: LA1.5, Nachprüfung 2026-11-21.
