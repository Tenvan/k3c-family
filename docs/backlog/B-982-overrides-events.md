# B-982 · Eine Welt übersteuert die Events der Nacht über overrides › events

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU60
- **Projekt:** RSU
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`data/events.json` (Vollmond, Blutmond, Händler-Überfall; `docs/rules/bosse.md` § 2) gilt für alle Welten gleich. `overrides` einer Welt erlaubt nur `buildings`, `ship`, `troops`, `economy`, `hub`, `monarch`, `enemies`, `waves`, `goals` (`docs/rules/welten.md` § 1.1, `engine/sim/world_values.go`); `events` wird als unbekannte Datei abgelehnt.

## Ziel

Eine Welt stellt ihre Events ein: Abstand, Stärke und Lohn je Event, oder ein Event ganz aus.

## Beteiligte und Zielgruppen

Spieler erleben je Welt andere Nächte. Welten-Autoren (🧑, Claude über `world_override_set`). 🧑 gibt frei (Spielregel).

## Anforderungen

- `events` ist eine erlaubte Override-Datei; Merge wie in `welten.md` § 1.1 (Liste von Objekten, Schlüssel = `id`, keine neuen Einträge).
- Ein Event ist aus, wenn sein Abstand 0 ist (`everyNights: 0` bzw. `everyVisits: 0`); in der Basis bleibt alles wie heute.
- Der Raum rechnet mit den Events des Wertesatzes der Welt, nicht mit der eingebetteten Basis; jede Stufe der Insel nutzt denselben Satz.
- Prüfung beim Laden (fail closed): Abstände ≥ 0, Faktoren > 0, `kind`/`elite` bekannte Gegnerarten; Meldung nach dem Muster aus `welten.md` § 1.2.
- Standard-Welt unverändert, `testdata/golden/` bytegleich; deterministisch; 2 Spieler ohne Sonderfall.
- `welten.md` § 1.1/§ 1.2 und `bosse.md` § 2 nachgezogen.

## Nicht-Ziele

Neue Events (B-991, B-992), Events je Insel oder Stufe, Anzeige im Editor (B-987).

## Regeln und Einschränkungen

Umgehängt auf Rust (B-1143, 2026-11-19): Sim in `rust/crates/k3c-sim`, Raum in `rust/crates/k3c-room` (Entscheidung 013); die Go-Engine bekommt keine Änderung mehr. Keine Simulation.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-room/**`, `docs/rules/welten.md`, `docs/rules/bosse.md`
- **Abhängig von:** B-981
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

`overrides: {"events": {"events": {"fullMoon": {"everyNights": 5}}}}` → Vollmond jede 5. Nacht in dieser Welt, Blutmond wie in der Basis. `bloodMoon.everyNights: 0` → kein Blutmond.

## Ausnahme- und Fehlerfälle

Neues Event per Override (`"taifun": {…}`) → abgelehnt (keine neuen Einträge). Negativer Abstand oder Faktor ≤ 0 → Welt lädt nicht, Meldung mit Pfad. Kind-Welt (`extends`) erbt die Event-Overrides der Eltern-Welt.

## Akzeptanzkriterien

- **AC-01** Test: Welt mit `overrides › events` lädt, der Raum nutzt den geänderten Abstand; Abstand 0 schaltet das Event aus.
- **AC-02** Tests der Fehlerfälle mit Meldung (neuer Eintrag, ungültiger Wert, unbekannte Gegnerart).
- **AC-03** `testdata/golden/` unverändert.
- **AC-04** `docs/rules/welten.md` und `bosse.md` nachgezogen.

## Offene Fragen

- Ausschalten → Default: Abstand 0 = aus, kein eigenes Feld `enabled` (vorläufig, 🧑 bestätigt).
- Neue Events je Welt → Default: stehen in der Basis mit Abstand 0 und werden je Welt eingeschaltet (vorläufig, 🧑 bestätigt).

## Notizen

Zerlegt aus B-959 (Punkt Overrides für `events`).
