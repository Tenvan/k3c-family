# B-910 · Abnahme BI2: Biom-Darstellung ist gesichtet, Grafik gerendert und Golden geprüft

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** RU61
- **Projekt:** RSU
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: nächste autonome Welle)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint BI2 hat Kristallfarben (B-900, B-901), Decke und Tropfsteine (B-902, B-903), Stützbalken und Verfall (B-904), Glut (B-905) und Leuchten (B-906) umgesetzt. Die Sessions liefen ohne Gen-Läufe und ohne Sichtprüfung (B-742).

## Ziel

Die Darstellung ist im laufenden nativen Spiel gesichtet, die Grafik gerendert und die Golden-Prüfungen grün.

## Beteiligte und Zielgruppen

Ralf 🧑 (Sichtprüfung), Claude (Gen-Läufe).

## Anforderungen

- `task gen:paper` rendert `ader-kristall-violett` und `ader-kristall-gruen`; Grafik-Golden (`public/grafik/objekte/golden.json`) stimmt.
- Die grafikabhängigen Tests in `native/k3c-game/props` laufen ohne Skip (`TestJedeGewaehlteGrafikLiegtVorUndIstGemeldet`).
- `task check:race` (ohne C-Compiler lokal übersprungen, dann so nennen).
- Sichtprüfung im nativen Spiel je Biom mit Werten min/max (Welten-Editor).

## Nicht-Ziele

Simulationen und Balancing (gesperrt).

## Regeln und Einschränkungen

Genau ein Abnahme-Ticket je Sprint (B-742). Manuelle Abnahme nur durch Ralf.

Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game`; die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `public/grafik/objekte/**`, `rust/crates/k3c-game/src/props/**`
- **Abhängig von:** BI2.5
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** golden

## Beispiele

Kristallhöhle mit `crystalColors` nur violett → alle Adern violett mit violettem Schein.

## Ausnahme- und Fehlerfälle

Wirkt eine Darstellung störend (z. B. Decke verdeckt Hintergrund zu stark) → Folge-Ticket im Projekt NAT mit Sprint, Werte in `props/decor*.go` anpassen.

## Akzeptanzkriterien

- **AC-01** `gen:paper` und Grafik-Golden grün, `TestJedeGewaehlteGrafikLiegtVorUndIstGemeldet` läuft ohne Skip grün; `check:race` grün oder als übersprungen genannt.
- **AC-02** 🧑 Ralf hat im nativen Spiel Kristallfarben (drei Farben), Decke und Tropfsteine (Höhle), Stützbalken mit Verfall (Mine), Glut (Eisenstollen) und Schein (Kristallhöhle) je bei kleinstem und größtem Wert gesichtet und abgenommen.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
