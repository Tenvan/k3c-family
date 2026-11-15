# B-1001 · Hütte, Figur und Bauwerk der Welt sind im Spiel sichtbar

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** SI5
- **Projekt:** KMP
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Teil von B-972 (Ideenrunde k3c-07, 2026-11-15): Die Engine kennt nach SI5.2 Hütte, Zahlung, Bauplatz `hermitHall` und Bonus. Bilder für Hütte, Figur und Bauwerk fehlen; Hub-Gebäude liegen als Papierschnitt in `public/grafik/gebaeude-paper/`, Figuren in `public/grafik/figuren/` und `truppen/`.

## Ziel

Die Spieler finden die Hütte der Figur, sehen sie nach der Zahlung in den Hub ziehen und dort ihr Bauwerk errichten.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions (SI5.3 Assets, SI5.4 Anzeige), Fachprüfung durch `k3c-grafik`. Abnahme am TV: 🧑.

## Anforderungen

- Anforderungen aus `B-972 › Anforderungen` gelten; hier nur Bild und Anzeige.
- Assets als Papierschnitt, nur über `assets_*`: `einsiedler-huette` (eine Hütte für alle Welten), Bauwerke `dojo` und `schrein` (Gruppe `gebaeude-paper`), Figuren `schwertschmied` und `priesterin` (Gruppe `figuren`).
- Vor der Zahlung steht die Figur an der Hütte, die Zahlstellen-Anzeige zeigt den Preis; danach geht die Figur sichtbar zum Abgang und erscheint im Hub am Bauplatz (nur Anzeige, der Bau läuft in der Engine sofort).
- Das Bauwerk nutzt die vorhandene Bau-Darstellung (Gerüst, dann Gebäude); zerstört und repariert wie andere Hub-Gebäude.
- Split-Screen: jede Hälfte zeigt, was in ihrer Stufe steht.

## Nicht-Ziele

Walfänger und Tranhütte (kommen mit der Polar-Welt), Anzeige des Bonus im HUD, neue Sounds, Dialog oder Text der Figur, Regeländerungen (B-972).

## Regeln und Einschränkungen

- **Bereich:** `public/grafik/gebaeude-paper/**`, `public/grafik/figuren/**`, `data/paper.json`, `native/k3c-game/**`
- **Abhängig von:** B-972
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –
- Zwei Sessions: SI5.3 Assets (PAP), SI5.4 Anzeige (CLI). Grafik-Golden und `gen:paper` laufen im Abnahme-Ticket des Sprints (B-742).

## Beispiele

- Sengoku, Stufe 2: Hütte mit Schwertschmied. Nach der Zahlung geht er zum Aufgang; im Hub entsteht am Platz `hermitHall` das Gerüst, dann der Dojo.
- Welt `standard`: keine Hütte, kein Bauplatz.

## Ausnahme- und Fehlerfälle

- Der zahlende Monarch bleibt in Stufe 2, der andere steht im Hub → dieser sieht Gerüst und Figur am Bauplatz, jener die leere Hütte.
- Katalog-ID ohne Zuordnung in `data/paper.json` → Test schlägt fehl.

## Akzeptanzkriterien

- **AC-01** Die Assets `einsiedler-huette`, `dojo`, `schrein` (Gruppe `gebaeude-paper`) sowie `schwertschmied` und `priesterin` (Gruppe `figuren`) sind über `assets_*` angelegt, in `data/paper.json` zugeordnet und von `k3c-grafik` ohne schweren Befund geprüft.
- **AC-02** Go-Test in `native/k3c-game`: Zustand vor der Zahlung → Hütte mit Figur und Preis; nach der Zahlung → leere Hütte, Figur am Bauplatz, Gerüst oder Bauwerk.
- **AC-03** Beobachtung am TV (🧑): Hütte in Stufe 2 gefunden, Zahlung, Figur zieht in den Hub, Bauwerk steht; beide Hälften stimmen.
- **AC-04** `check_run task:check:diff` grün.

## Offene Fragen

Mit Default entschieden (vorläufig, 🧑 bestätigt):

- Eine Hütte je Welt oder eine für alle? → Eine für alle Welten; die Figur zeigt die Welt.
- Bonus sichtbar? → Nur über das Bauwerk, keine HUD-Anzeige.

## Notizen

Am 2026-11-15 aus B-972 abgetrennt (Zerlegung von L): Regel und Engine bleiben in B-972.
