# B-373 · Der Händler ist eine angreifbare Figur mit Besuchszähler im Spielstand

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** K3
- **Projekt:** KMP
- **Erstellt:** 2026-10-09
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-09, Chat, durch 🧑, mit Sprint K3 Revision 2

## Ausgangslage

K3.2 (Händler-Überfall, B-131/AC-02) baut laut Session auf dem Händler aus W4.2 auf. `engine/sim/merchant.go` modelliert ihn nur als Zustand `World.Merchant` (Material, Abreisetag, laufender Kauf): keine Figur mit HP, kein Ziel für Gegner (`candidates`/`applyDamage`), kein Zähler der Besuche (der Rhythmus wechselt mit der Taverne zwischen 3 und 2 Tagen, ist also nicht aus `Cycle.Day` ableitbar), und weder `Merchant` noch ein Zähler stehen im Spielstand (`island_save.go`). K3.2 nennt den Händler selbst als Nicht-Ziel; deshalb ist K3.2 blockiert.

## Ziel

Der Händler ist während seines Besuchs eine Figur, die Gegner angreifen können; die Insel zählt seine Besuche, und der Zähler überlebt Speichern und Laden. Danach kann K3.2 den Überfall daran hängen.

## Beteiligte und Zielgruppen

Spieler (Händler schützen), 🧑 entscheidet die offenen Fragen.

## Anforderungen

- Händler-HP und Ort in den Daten (`data/economy.json › merchant` oder `hub.json`), Schaden über `applyDamage`, Ziel in `candidates`.
- Stirbt er, reist er sofort ab (Ereignis), der laufende Kauf fällt als Münzen.
- Besuchszähler je Insel (`Island`), +1 bei Ankunft; Spielstand-Version + 1 mit Fixture und Migrationstest; anwesender Händler wird mitgespeichert oder geht beim Laden bewusst verloren (Entscheidung unten).

## Nicht-Ziele

Der Überfall selbst und seine Belohnung (K3.2), Anzeige (K5).

## Regeln und Einschränkungen

Domäne SIM; Werte nur in `data/`; deterministisch; Spielstand nach `docs/arbeitsweise.md` › Spielstand-Format ändern. Quelle: `docs/rules/bosse.md` § 2, `docs/rules/buerger.md` § 1.

## Beispiele

Händler kommt an Tag 3 (Besuch 1), Tag 6 (2), Taverne gebaut, Tag 8 (3), Tag 10 (4 → Überfall in K3.2).

## Ausnahme- und Fehlerfälle

Burgfall während des Besuchs: Händler bleibt bzw. reist ab laut Entscheidung; Laden eines alten Stands: Zähler 0.

## Akzeptanzkriterien

- **AC-01** Test: Ein Gegner in Reichweite greift den anwesenden Händler an, seine HP sinken; bei 0 reist er ab (Ereignis), ohne Händler gibt es kein Ziel.
- **AC-02** Test: Der Besuchszähler steigt je Ankunft um 1 (mit und ohne Taverne) und übersteht Speichern und Laden; Stände der Vorversion laden mit Zähler 0.

## Offene Fragen

Entschieden von 🧑 am 2026-10-09 (Chat): Händler-HP 100 (vorläufig, BR2); ein anwesender Händler wird mit HP und Abreisetag gespeichert; Umsetzung als Session K3.2a vor K3.2. Heilen durch Spieler oder Truppen: nicht vorgesehen.

## Notizen

Aus K3.2 (2026-10-09), blockiert diese Session.
