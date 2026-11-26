# B-990 · Britannien bekommt die Assets Steinkreis, Rundhaus, römischer Späher und Moorgeist

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WN2
- **Projekt:** WZN
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Britannien (WE3) nutzt nur vorhandene Assets. Papierschnitt-Assets entstehen aus Specs unter `public/grafik/<gruppe>/` und nur über `assets_*` (Entscheidung 005).

## Ziel

Britannien hat eigene Bilder: Steinkreis, Rundhaus, römischer Späher und Moorgeist liegen als Papierschnitt-Assets vor und sind als Skins nutzbar.

## Beteiligte und Zielgruppen

Spieler sehen die Welt. Claude legt die Assets über `assets_*` an, Agent `k3c-grafik` prüft, 🧑 nimmt im Bereich Assets ab.

## Anforderungen

- Assets: `objekte/steinkreis` (Ersatz für `objekte/portal`), `gebaeude-paper/rundhaus` (Ersatz für das Bauernhaus), `gegner/roemischer-spaeher` (Basis `gegner/goblin-bogenschuetze`), `gegner/moorgeist` (Basis `gegner/grubengeist`).
- Jedes Skin-Asset hat dieselben Zustände, Animationen, Ankerpunkte und etwa dieselbe Größe wie sein Basis-Asset (B-994).
- Stil nach `assets_guide` und `assets_style_get`; Texturbudget eingehalten.
- Nur über `assets_*`; Specs und `index.paper.json` eingecheckt, PNG nicht (B-737).

## Nicht-Ziele

Zuordnung in der Welt (B-996), Grafik-Golden und `gen:paper` (Abnahme-Ticket), neue Animationen.

## Regeln und Einschränkungen

Entscheidung 005. Gen-Läufe nur im Abnahme-Ticket (B-742).

- **Bereich:** `public/grafik/objekte/**`, `public/grafik/gegner/**`, `public/grafik/gebaeude-paper/**`
- **Abhängig von:** B-989
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

`assets_get gegner/roemischer-spaeher` zeigt die Animationen des Goblin-Bogenschützen (Fernkampf). `assets_preview objekte/steinkreis` zeigt einen Steinkreis in Portal-Größe.

## Ausnahme- und Fehlerfälle

Das Portal hat Zustände (aktiv, zerstört) → der Steinkreis bildet beide ab. Texturbudget überschritten → Asset verkleinern.

## Akzeptanzkriterien

- **AC-01** `assets_list` zeigt die vier Assets; Specs liegen unter `public/grafik/`.
- **AC-02** Je Skin-Asset: Zustände und Animationen deckungsgleich mit dem Basis-Asset (im Ergebnis notiert).
- **AC-03** Agent `k3c-grafik` ohne schwere Befunde.
- **AC-04** Sichtprüfung 🧑 im Bereich Assets.

## Offene Fragen

- Basis je Asset → Default: Steinkreis = Portal, Rundhaus = Bauernhaus, römischer Späher = Goblin-Bogenschütze, Moorgeist = Grubengeist (vorläufig, 🧑 bestätigt).

## Notizen

Zerlegt aus B-960 (Assets und Gegner-Varianten Britannien). Abhängigkeit von B-989 nur wegen der gemeinsamen Gruppen-Indizes.
