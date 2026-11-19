# B-967 · Vor der Abfahrt wählen die Spieler die Fracht des Schiffs

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU62
- **Projekt:** RSU
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Heute beginnt die neue Insel nach dem Inselwechsel mit dem Startvorrat von 100 Holz (`docs/rules/stufen.md` § 1, B-177); eine Wahl, was mitfährt, gibt es nicht.

## Ziel

Vor der Abfahrt wählen die Spieler, was mitfährt: ein paar Truppen, Münzen oder Material. Der Inselwechsel wird zur Entscheidung statt zum Neustart bei null.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions (SI1). Freigabe der Spec, Bestätigung der Defaults und Abnahme am TV: 🧑.

## Anforderungen

Dieses Ticket trägt **Regeltext und Engine**; Assets und Anzeige stehen in B-997.

- **Laderaum mit Plätzen (🧑 2026-11-15):** Das Schiff hat wenige Plätze (Startwert offen, z. B. 6). Jeder Platz nimmt eine Truppe, einen Beutel Münzen oder eine Kiste Material; die Spieler entscheiden die Mischung.
- **Beladen in der Welt am Steg (🧑 2026-11-15):** Kein Menü. Am Steg stehen Ladeplätze: Münzen einwerfen lädt einen Beutel, Truppen gehen per Befehl an Bord, Bauern tragen Material aus dem Vorrat. Beide Spieler laden gleichzeitig, die Fracht ist an Deck sichtbar.
- **Mehr Plätze durch besseres Schiff (🧑 2026-11-15):** Der Startvorrat der neuen Insel bleibt. Das Grundschiff (Kiel, Rumpf, Segel) hat wenige Plätze; ein vierter, freiwilliger Bauabschnitt „Laderaum“ vergrößert es.
- Die Fracht kommt auf der neuen Insel zum Startvorrat hinzu; Zurückgelassenes bleibt auf der alten Insel (keine Rückkehr).
- Funktioniert mit 2 Spielern gleichzeitig.

Ergänzt (Defaults, siehe Offene Fragen):

- Drei Ladeplätze am Steg des Hafens: Münzen, Truppen, Material. Sie sind nutzbar, sobald das Grundschiff (Segel) fertig ist, auch vor dem Sieg über den Endgegner.
- Münz-Ladeplatz: Ein Monarch wirft `cargo.purseCoins` Münzen ein, dann belegt ein Beutel einen Platz.
- Truppen-Ladeplatz: Ein Münzeinwurf ruft die nächststehende Truppe der Stufe (keine Bürger) an Bord; sie belegt einen Platz und kämpft nicht mehr.
- Material-Ladeplatz: Ein Münzeinwurf beauftragt einen Bauern, eine Kiste (`cargo.crateUnits`) des Materials mit dem größten Vorrat der Insel zum Steg zu tragen; die Kiste belegt den Platz erst, wenn sie an Bord ist, das Material ist ab Auftrag reserviert.
- Geladenes bleibt an Bord (kein Entladen).
- Ankunft: Material kommt in den Vorrat der neuen Insel (zusätzlich zum Startvorrat), Truppen stehen am Hafen der neuen Insel (zusätzlich zu den Start-Truppen), Beutel liegen als Münzen am Steg und sind für alle Monarchen aufsammelbar.
- Der Abschnitt „Laderaum“ ist ein vierter Eintrag in `data/ship.json` mit `optional: true`; die Abfahrt verlangt ihn nicht.
- Deterministisch: Laden in fester Reihenfolge nach Spieler-Index, kein Zufall außer `engine/rng`.
- Alle Zahlen stehen in `data/ship.json` und sind vorläufig (Balancing später).

## Nicht-Ziele

Lade-Menü oder Overlay, automatische Beladung nach Profil, Wegfall des Startvorrats.

## Regeln und Einschränkungen

- Umgehängt auf Rust (B-1143, 2026-11-19): Sim in `rust/crates/k3c-sim`, Raum in `rust/crates/k3c-room` (Entscheidung 013); die Go-Engine bekommt keine neue Mechanik. Die Go-Sessions des Sprints SI1 sind verworfen; Sessions entstehen beim Aktivieren von RU62 (Regeltext REG, Sim SIM).
- Keine Simulation, kein Balancing (gesperrt); Werte sind vorläufig. Keine Rücksicht auf alte Spielstände.
- Koop-Werte nur über Multiplikator-Felder; die Platzzahl hat vorläufig keinen Koop-Multiplikator.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `docs/rules/stufen.md`, `docs/rules/wirtschaft.md`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-room/**`, `data/ship.json`
- **Abhängig von:** B-961, RU37
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** spielstand

## Beispiele

- Grundschiff mit 6 Plätzen: 3 Schützen, 2 Beutel Münzen, 1 Kiste Holz → Insel 2 beginnt mit 100 Holz Startvorrat plus dieser Fracht. Mit Laderaum passen mehr Plätze an Bord.
- Beide Monarchen werfen im selben Tick am Münz-Ladeplatz ein, ein Platz ist frei → Spieler 1 lädt den Beutel, die Münzen von Spieler 2 bleiben bei ihm.
- Schiff ohne Laderaum, alle Monarchen am Steg → Abfahrt wie heute, auch mit leeren Plätzen.

## Ausnahme- und Fehlerfälle

- Laderaum voll → Einwurf wird abgelehnt (keine Münze abgezogen), Ereignis `cargoFull`.
- Material-Ladeplatz, Vorrat unter einer Kiste → abgelehnt, keine Münze abgezogen.
- Keine Truppe auf der Stufe → abgelehnt.
- Abfahrt, während ein Bauer noch eine Kiste trägt → die Kiste fährt nicht mit, das reservierte Material geht in den Vorrat zurück (bleibt auf der alten Insel).
- Das Schiff verliert den Abschnitt Laderaum (Stufenverlust): Plätze über der neuen Platzzahl werden in umgekehrter Lade-Reihenfolge zurückgegeben (Material in den Vorrat, Truppen an Land, Münzen als Münzen am Steg).
- Spielstand speichern und laden während des Beladens → Fracht und offene Träger-Aufträge bleiben erhalten.
- Letzte Insel der Welt (kein Wechselpunkt) → keine Ladeplätze.

## Akzeptanzkriterien

- **AC-01** `docs/rules/stufen.md` § 1 und `docs/rules/wirtschaft.md` § 8 beschreiben Laderaum, Plätze, Ladeplätze, Beutel, Kiste, Ankunft und den Abschnitt „Laderaum“ in der Tabellenform der Datei (Regel, Begründung, Daten, Zielkorridor); Werte als vorläufig markiert.
- **AC-02** `data/ship.json` hat `cargo` (`slots`, `holdSlots`, `purseCoins`, `crateUnits`) und einen vierten Abschnitt `hold` mit `optional: true`; ein Go-Test lädt die Datei und lehnt fehlende oder negative Werte ab.
- **AC-03** Go-Test: Münzen, Truppe und Material belegen je einen Platz; bei vollem Laderaum wird abgelehnt, nichts abgezogen, Ereignis `cargoFull`.
- **AC-04** Go-Test: Zwei Spieler laden im selben Tick; das Ergebnis ist bei gleichem Seed und gleichen Eingaben gleich, bei einem freien Platz gewinnt der kleinere Spieler-Index.
- **AC-05** Go-Test: Nach `NextIsland` hat die neue Insel Startvorrat plus Fracht-Material im Vorrat, die Fracht-Truppen am Hafen und die Beutel als Münzen am Steg; nichts Zurückgelassenes ist vorhanden.
- **AC-06** Go-Test: Abfahrt ohne Abschnitt `hold` ist möglich; mit `hold` ist die Platzzahl `slots + holdSlots`; Verlust des Abschnitts gibt überzählige Fracht zurück.
- **AC-07** Go-Test: Fracht und Träger-Aufträge überstehen Speichern und Laden (`IslandSaveVersion` erhöht, Fixtures angepasst).
- **AC-08** Zustand und Ereignisse nennen Plätze, Belegung je Platz und Ladeplätze, sodass B-997 sie zeichnen kann (Test auf die Felder).
- **AC-09** `check_run task:check:diff` grün; `testdata/golden/` unverändert oder Änderung im Commit begründet.

## Offene Fragen

Alle mit Default entschieden (vorläufig, 🧑 bestätigt):

- Wie viele Plätze, was kostet der Laderaum? → Grundschiff 6, Laderaum +4; Kosten `hold`: 25 Münzen, 60 Holz, 40 Stein, 16 s Bauzeit (vorläufig, Balancing später).
- Wie viel fasst Beutel und Kiste? → Beutel 20 Münzen, Kiste 50 Einheiten eines Materials (vorläufig).
- Fahren die Monarchen-Münzen im eigenen Beutel ohnehin mit? → Ja, das heutige Verhalten beim Inselwechsel bleibt unverändert (Code ungeprüft, SI1.2 prüft `engine/sim/island_travel.go` und hält den Befund im Ergebnis fest).
- Welches Material lädt der Material-Ladeplatz? → Das mit dem größten Vorrat; Alternative wäre ein Ladeplatz je Material (mehr Platz am Steg).
- Wie gehen Truppen „per Befehl“ an Bord? → Münzeinwurf am Truppen-Ladeplatz ruft die nächststehende Truppe; einen allgemeinen Truppen-Befehl gibt es nicht (geprüft: `engine/sim/unit_escort.go` ist nur der Begleitschutz zum Endboss).
- Entladen? → Nein, Geladenes bleibt an Bord.
- Wo landen die Beutel? → Als Münzen am Steg der neuen Insel, für alle aufsammelbar.

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt. 2026-11-15 von L in B-967 (Regel und Engine) und B-997 (Assets und Anzeige) zerlegt; das Reittier belegt keinen Platz (B-970).
