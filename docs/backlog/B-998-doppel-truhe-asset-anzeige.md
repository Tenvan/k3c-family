# B-998 · Die Doppel-Truhe hat ein eigenes Bild und zeigt, welcher Monarch fehlt

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
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

Teil von B-968 (Ideenrunde k3c-07, 2026-11-15): Die Engine kennt nach SI2.3 die Doppel-Truhe und meldet, welcher Monarch davor wartet. Vorhanden sind die Truhe als Papierschnitt (`public/grafik/objekte/truhe`, `truhe-offen`) und die Münz-Fontäne im Spiel (`native/k3c-game/props/coin_fountain.go`, B-953).

## Ziel

Die Doppel-Truhe ist als solche erkennbar (zwei Schlösser) und zeigt, dass der zweite Monarch fehlt.

## Beteiligte und Zielgruppen

Spieler: zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions (SI2.2 Asset, SI2.4 Anzeige), Fachprüfung durch `k3c-grafik`. Abnahme am TV: 🧑.

## Anforderungen

- Anforderungen aus `B-968 › Anforderungen` gelten; hier nur Bild und Anzeige.
- Asset `doppel-truhe` mit zwei Schlössern (zu, ein Schloss leuchtet links, ein Schloss leuchtet rechts) und `doppel-truhe-offen`, Papierschnitt, nur über `assets_*`.
- Wirkt das Kennzeichen nicht (ein gesteuerter Monarch), zeichnet das Spiel an der Stelle die normale Truhe.
- Steht ein Monarch davor, leuchtet ein Schloss; das zweite bleibt dunkel und zeigt, dass der zweite fehlt (🧑 2026-11-15).
- Öffnen zeigt die vorhandene Fontäne, in beiden Hälften.

## Nicht-Ziele

Neue Sounds, HUD-Text, Wegweiser zur Truhe, Regeländerungen (B-968).

## Regeln und Einschränkungen

- Umgehängt auf Rust (B-1143, 2026-11-19): Anzeige in `rust/crates/k3c-game`; die Go-Sessions des Sprints SI2 sind verworfen; Sessions entstehen beim Aktivieren von RU62 (Asset PAP, vor der Sim möglich; Anzeige CLI). Grafik-Golden und `gen:paper` laufen im Abnahme-Ticket des Sprints (B-742).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `public/grafik/objekte/**`, `data/paper.json`, `rust/crates/k3c-game/src/**`
- **Abhängig von:** B-968, RU37
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

- Monarch 1 steht davor → linkes Schloss leuchtet. Monarch 2 kommt dazu → Truhe offen, Fontäne.
- Spieler 2 verlässt das Spiel → an der Stelle steht die normale Truhe.

## Ausnahme- und Fehlerfälle

- Wechsel zwischen einem und zwei gesteuerten Monarchen bei sichtbarer Truhe → das Bild wechselt ohne Übergang zwischen Truhe und Doppel-Truhe.
- Asset fehlt in `data/paper.json` → Test schlägt fehl.

## Akzeptanzkriterien

- **AC-01** Die Assets `doppel-truhe` und `doppel-truhe-offen` (Gruppe `objekte`) sind über `assets_*` angelegt, in `data/paper.json` zugeordnet und von `k3c-grafik` ohne schweren Befund geprüft.
- **AC-02** Go-Test in `native/k3c-game`: wirkendes Kennzeichen → Doppel-Truhe, sonst normale Truhe; wartender Monarch → das zugehörige Schloss leuchtet.
- **AC-03** Beobachtung am TV (🧑): Zu zweit öffnet die Truhe nur gemeinsam, die Fontäne ist in beiden Hälften zu sehen; Solo steht dort eine normale Truhe.
- **AC-04** `check_run task:check:diff` grün.

## Offene Fragen

Mit Default entschieden (vorläufig, 🧑 bestätigt):

- Welches Schloss leuchtet? → Links für den Monarchen mit dem kleineren Spieler-Index, rechts für den anderen.
- Eigener Ton? → Nein, der Ton der normalen Truhe.

## Notizen

Am 2026-11-15 aus B-968 abgetrennt: Regel und Engine bleiben in B-968.
