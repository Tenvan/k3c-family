# B-991 · Das Event Überfall bringt jede n-te Nacht eine zusätzliche Gruppe einer Gegnerart (Ninja-Überfall in Sengoku)

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WE8
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`data/events.json` kennt Vollmond (`fullMoon`: jede n-te Nacht zusätzliche Gegner der Art `kind` plus ein Elite-Gegner, Lohn bei Tagesanbruch), Blutmond und Händler-Überfall (`docs/rules/bosse.md` § 2, `engine/sim/events.go`, `events_moon.go`). Ein Event, das eine Welt mit ihrer eigenen Gegner-Variante prägt, fehlt.

## Ziel

Eine Welt kann einen Überfall haben: Jede n-te Nacht kommt zusätzlich eine schnelle Gruppe einer festgelegten Gegnerart. In Sengoku ist das der Ninja-Überfall.

## Beteiligte und Zielgruppen

Spieler (2 Spieler) verteidigen die Burg. 🧑 gibt frei (neue Spielregel).

## Anforderungen

- Neues Event `raid` in `data/events.json`: `everyNights`, `kind` (Gegnerart), `share` (Anteil an der Wellengröße, aufgerundet, mindestens 1), `speedFactor`, `rewardGold`. In der Basis `everyNights: 0` (aus); eine Welt schaltet es über `overrides › events` ein (B-982).
- Die Gruppe kommt in der Event-Nacht zusätzlich zur Welle der Stufe über die Portale der Oberfläche (Tiefe 0), mit Tempo × `speedFactor`; sonst gelten die Werte der Art.
- Hält die Burg bis Tagesanbruch, gibt es `rewardGold` wie beim Vollmond.
- Fällt der Überfall auf eine Vollmond- oder Blutmond-Nacht, gelten beide Events.
- Deterministisch (nur `engine/rng`), gleiche Wirkung für alle Spieler der Insel; Zustand und Ereignis nennen das Event (`docs/protocol.md`).
- Regeltext in `bosse.md` § 2, Begriff „Überfall“ im Glossar.
- Standard-Welt unverändert, `testdata/golden/` bytegleich.

## Nicht-Ziele

Gegner, die Mauern überspringen oder unsichtbar sind, Anzeige im Spiel (B-995), Einschalten in Sengoku (B-996), Balancing.

## Regeln und Einschränkungen

Umgehängt auf Rust (B-1143, 2026-11-19): Sim in `rust/crates/k3c-sim`, Raum in `rust/crates/k3c-room` (Entscheidung 013); die Go-Engine bekommt keine neue Mechanik. Werte in `data/events.json`, vorläufig. Keine Simulation.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-room/**`, `data/events.json`, `docs/rules/bosse.md`, `docs/protocol.md`
- **Abhängig von:** B-982, RU37
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** protokoll

## Beispiele

Sengoku: `raid` mit `everyNights: 5`, `kind: wolf` (dort „Ninja“), `share: 0.5`, `speedFactor: 1.3` → in Nacht 5, 10, 15 kommt zur Welle eine halbe Wellengröße schneller Ninja dazu. Standard-Welt → kein Überfall.

## Ausnahme- und Fehlerfälle

Alle Portale der Oberfläche zerstört → kein Überfall in dieser Nacht (wie beim Miniboss, `kampf.md` § 2). `kind` unbekannt oder `share`/`speedFactor` ≤ 0 → Daten bzw. Welt laden nicht. Spielstand mitten in der Event-Nacht geladen → das Event läuft weiter wie der Vollmond.

## Akzeptanzkriterien

- **AC-01** Test: mit eingeschaltetem `raid` kommt in der n-ten Nacht die Zusatzgruppe in erwarteter Zahl und mit erhöhtem Tempo; in anderen Nächten nicht.
- **AC-02** Test: Lohn bei gehaltener Burg; kein Überfall ohne Portale; Überlagerung mit Vollmond.
- **AC-03** Test: gleicher Seed → gleicher Ablauf; `testdata/golden/` unverändert.
- **AC-04** `bosse.md` § 2 und `docs/protocol.md` nachgezogen.

## Offene Fragen

- Mechanik → Default: allgemeines Event `raid` nach dem Muster des Vollmonds, kein Sonderverhalten der Ninja (vorläufig, 🧑 bestätigt).
- Werte → Default: `everyNights` 5, `share` 0,5, `speedFactor` 1,3, `rewardGold` 9 (vorläufig, Balancing später, 🧑 bestätigt).
- Spielstand → Default: kein neues Feld nötig, das Event ergibt sich aus Nacht und Daten; braucht es doch eines, stoppt die Session und meldet es (Sperre `spielstand`).

## Notizen

Zerlegt aus B-960 (Event Ninja-Überfall).
