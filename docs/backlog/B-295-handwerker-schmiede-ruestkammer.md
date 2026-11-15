# B-295 · Handwerker lassen sich auch für Schmiede und Rüstkammer ausbilden

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RG2
- **Projekt:** BAL
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

Seit WI1 (B-950) heißt der Bauer mit Hammer **Handwerker** (baut und repariert); der frühere Beruf `craftsman` heißt **Geselle** (`docs/glossar.md` Zeile 105 und 115). Dieses Ticket meint den Gesellen. `docs/rules/buerger.md` § 1 (Zeile 21), § 2 (Zeile 38) und das Glossar (Zeile 105) nennen ihn „in Werkstatt, Schmiede oder Rüstkammer (1–2 je Gebäude)“. In `data/buildings.json` gibt es das Angebot `craftsman` nur an der Werkstatt (`dx −4`); Schmiede (nur `eliteWarrior`), Rüstkammer und Bogner haben keins. Die Ausbildung bindet den Gesellen an das Gebäude seines Angebots (`engine/sim/unit_professions.go`), deshalb beschleunigt kein Geselle Elite-Upgrade und Rüstung. Entschieden am 2026-10-31 (Claude im Auftrag von Ralf, B-627 › Notizen): **nur in der Werkstatt**.

## Ziel

Regeltext, Glossar und Daten sagen dasselbe: Gesellen gibt es nur an der Werkstatt.

## Beteiligte und Zielgruppen

Spieler (Koop); 🧑 bestätigt die Entscheidung; ein Agent gleicht den Text an.

## Anforderungen

- `docs/rules/buerger.md` § 1 und § 2 nennen den Gesellen nur an der Werkstatt (höchstens 2); er beschleunigt nur, was die Werkstatt herstellt.
- Die Herstellungszeiten in § 2 sagen, dass Elite-Upgrade (Schmiede), Rüstungsstufe (Rüstkammer) und Bogen (Bogner) ohne Gesellen-Bonus laufen.
- `docs/glossar.md` › Geselle passt dazu (Sammeldatei, letzter Commit des Sprints).
- `data/buildings.json` bleibt unverändert; gilt für jeden Spieler gleich, keine Wirkung auf die Simulation.

## Nicht-Ziele

Neue Angebote oder `dx`-Werte an Schmiede, Rüstkammer oder Bogner; Balancing der Kosten und Zeiten (gesperrt); Code.

## Regeln und Einschränkungen

Domäne REG, nur Text. Begriffe nach `docs/glossar.md` (Geselle, nicht Handwerker).

- **Bereich:** `docs/rules/buerger.md`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Größe:** S
- **Sperre:** –

## Beispiele

Schmiede gebaut, Geselle an der Werkstatt ausgebildet → Schwert und Hammer entstehen schneller, das Elite-Upgrade dauert weiter 20 s.

## Ausnahme- und Fehlerfälle

🧑 will doch Gesellen an Schmiede und Rüstkammer → dieses Ticket endet ohne Textänderung, ein neues Ticket (REG Werte `dx`, dann SIM-Prüfung) übernimmt.

## Akzeptanzkriterien

- **AC-01** `grep -n "Geselle" docs/rules/buerger.md docs/glossar.md` nennt als Ort nur die Werkstatt; `data/buildings.json` hat `craftsman` weiter nur unter `workshop.offers` (unverändert, `git diff --stat data/` leer).

## Offene Fragen

- Gesellen auch an Schmiede und Rüstkammer? → Default: nein, nur Werkstatt (vorläufig, 🧑 bestätigt; Entscheidung B-627 vom 2026-10-31, keine neuen `dx`-Werte, keine Balancing-Last).
- Gilt das auch für den Bogner (neu seit B-951)? → Default: ja, Bogner ohne Geselle, wie `data/buildings.json` heute (vorläufig, 🧑 bestätigt).

## Notizen

Gefunden in W4.2 (Sprint W4). Titel stammt aus der Zeit vor B-950 („Handwerker“ = heute Geselle). Stand geprüft 2026-11-15: Text weiter widersprüchlich (`buerger.md`:21, :38, `glossar.md`:105).
