# B-989 · Nil bekommt die Assets Palme, Barke, Obelisk, Skorpion und Mumie

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WE7
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Nil (WE3) nutzt nur vorhandene Assets. Papierschnitt-Assets entstehen aus Specs unter `public/grafik/<gruppe>/` und nur über `assets_*` (Entscheidung 005).

## Ziel

Nil hat eigene Bilder: Palme, Barke, Obelisk, Skorpion und Mumie liegen als Papierschnitt-Assets vor und sind als Skins nutzbar.

## Beteiligte und Zielgruppen

Spieler sehen die Welt. Claude legt die Assets über `assets_*` an, Agent `k3c-grafik` prüft, 🧑 nimmt im Bereich Assets ab.

## Anforderungen

- Assets: `objekte/palme` (Ersatz für `objekte/tanne`, `objekte/birke`, `objekte/pappel`), `objekte/obelisk` (Ersatz für `objekte/felsen`), Barke (Ersatz für das Schiff, in der Gruppe des Schiff-Assets), `gegner/skorpion` (Basis `gegner/wolf`), `gegner/mumie` (Basis `gegner/zombie`).
- Jedes Skin-Asset hat dieselben Zustände, Animationen, Ankerpunkte und etwa dieselbe Größe wie sein Basis-Asset (B-994); die Barke hat dieselben Baustufen wie das Schiff.
- Stil nach `assets_guide` und `assets_style_get`; Texturbudget eingehalten.
- Nur über `assets_*`; Specs und `index.paper.json` eingecheckt, PNG nicht (B-737).

## Nicht-Ziele

Zuordnung in der Welt (B-996), Grafik-Golden und `gen:paper` (Abnahme-Ticket), neue Animationen.

## Regeln und Einschränkungen

Entscheidung 005. Gen-Läufe nur im Abnahme-Ticket (B-742).

- **Bereich:** `public/grafik/objekte/**`, `public/grafik/gegner/**`, `public/grafik/gebaeude-paper/**`
- **Abhängig von:** B-988
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

`assets_get objekte/palme` zeigt dieselben Zustände wie `objekte/tanne`. `assets_get gegner/mumie` zeigt die Animationen des Zombies.

## Ausnahme- und Fehlerfälle

Das Schiff ist kein einzelnes Asset, sondern aus Baustufen zusammengesetzt → die Barke bildet jede Stufe ab; geht das nicht in einer Session, Stufen in ein Folgeticket und im Ergebnis melden.

## Akzeptanzkriterien

- **AC-01** `assets_list` zeigt die fünf Assets; Specs liegen unter `public/grafik/`.
- **AC-02** Je Skin-Asset: Zustände und Animationen deckungsgleich mit dem Basis-Asset (im Ergebnis notiert).
- **AC-03** Agent `k3c-grafik` ohne schwere Befunde.
- **AC-04** Sichtprüfung 🧑 im Bereich Assets.

## Offene Fragen

- Basis je Asset → Default: Palme = Bäume, Obelisk = Felsen, Barke = Schiff, Skorpion = Wolf, Mumie = Zombie (vorläufig, 🧑 bestätigt).

## Notizen

Zerlegt aus B-960 (Assets und Gegner-Varianten Nil). Abhängigkeit von B-988 nur wegen der gemeinsamen Gruppen-Indizes.
