# B-997 · Ladeplätze am Steg und Fracht an Deck sind im Spiel sichtbar

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

Teil von B-967 (Ideenrunde k3c-07, 2026-11-15): Die Engine kennt nach SI1.2 Laderaum, Plätze und Ladeplätze am Steg, das Spiel zeichnet sie noch nicht. Vorhanden sind Hafen und Schiff als Papierschnitt (`public/grafik/gebaeude-paper/hafen`, `schiff`) und die Zahlstellen-Anzeige (`public/grafik/icons/zahlstelle-*`).

## Ziel

Beide Spieler sehen am Steg, wo sie Münzen, Truppen und Material laden, wie viele Plätze frei sind und was an Deck liegt.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions (SI1.3 Assets, SI1.4 Anzeige), Fachprüfung durch die Agenten `k3c-grafik` und `k3c-hud`. Abnahme am TV: 🧑.

## Anforderungen

- Anforderungen aus `B-967 › Anforderungen` gelten; hier nur Bild und Anzeige: kein Menü, kein Overlay, die Fracht ist an Deck sichtbar (🧑 2026-11-15).
- Assets als Papierschnitt (Entscheidung 005), nur über `assets_*`: drei Ladeplätze (Münzen, Truppen, Material), Beutel, Kiste, Schiff mit Abschnitt Laderaum.
- Am Steg stehen die drei Ladeplätze mit Zahlstellen-Anzeige wie andere Bauplätze; belegte und freie Plätze sind an Deck abzählbar (je Platz ein Beutel, eine Kiste oder eine Truppe).
- Volles Schiff und abgelehnter Einwurf geben eine sichtbare und hörbare Rückmeldung (vorhandener Ablehn-Ton, kein neuer Sound).
- Split-Screen: beide Hälften zeigen denselben Stand; nichts hängt an einem Spieler-HUD.
- Das Spiel zeichnet nur; alle Zustände kommen aus der Engine (B-967/AC-08).

## Nicht-Ziele

Lade-Menü oder Overlay, neue Sounds, Animation des Tragens über das vorhandene Bauern-Tragen hinaus, Regeländerungen (B-967).

## Regeln und Einschränkungen

- Umgehängt auf Rust (B-1143, 2026-11-19): Anzeige in `rust/crates/k3c-game`; die Go-Sessions des Sprints SI1 sind verworfen; Sessions entstehen beim Aktivieren von RU62 (Assets PAP, Anzeige CLI). Grafik-Golden und `gen:paper` laufen im Abnahme-Ticket des Sprints (B-742).
- Texturbudget und Stil prüft `k3c-grafik`; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `public/grafik/objekte/**`, `public/grafik/gebaeude-paper/**`, `data/paper.json`, `rust/crates/k3c-game/src/**`
- **Abhängig von:** B-967, RU37
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

- 6 Plätze, 3 Schützen, 2 Beutel, 1 Kiste geladen → an Deck stehen 3 Schützen, liegen 2 Beutel und 1 Kiste; kein freier Platz ist markiert.
- Monarch 2 wirft am vollen Schiff ein → Ablehn-Rückmeldung am Ladeplatz, in beiden Hälften sichtbar, wenn der Steg im Bild ist.

## Ausnahme- und Fehlerfälle

- Asset fehlt in `data/paper.json` → Test schlägt fehl (kein stiller Platzhalter).
- Schiff ohne Laderaum → nur die Grundplätze werden gezeichnet.
- Letzte Insel der Welt → keine Ladeplätze im Bild.

## Akzeptanzkriterien

- **AC-01** Die Assets `ladeplatz-muenzen`, `ladeplatz-truppen`, `ladeplatz-material`, `fracht-beutel`, `fracht-kiste` (Gruppe `objekte`) und die Laderaum-Stufe des Schiffs (`gebaeude-paper/schiff`) sind über `assets_*` angelegt, in `data/paper.json` zugeordnet und von `k3c-grafik` ohne schweren Befund geprüft.
- **AC-02** Go-Test in `native/k3c-game`: Aus einem Engine-Zustand mit Ladeplätzen und Fracht entstehen die erwarteten Zeichen-Einträge (Ladeplätze am Steg, je belegtem Platz ein Objekt an Deck).
- **AC-03** Go-Test: Ereignis `cargoFull` löst die Ablehn-Rückmeldung aus.
- **AC-04** Beobachtung am TV (🧑): Zwei Spieler beladen gleichzeitig, beide Hälften zeigen dieselbe Fracht; nach der Abfahrt liegt sie auf der neuen Insel.
- **AC-05** `check_run task:check:diff` grün.

## Offene Fragen

Mit Default entschieden (vorläufig, 🧑 bestätigt):

- Wie sehen die Ladeplätze aus? → Pfahl mit Schild (Münze, Schwert, Kiste) im Stil der vorhandenen Schilder (`objekte/schild-*`).
- Eigene Sounds? → Nein, vorhandene Töne für Zahlen und Ablehnen.

## Notizen

Am 2026-11-15 aus B-967 abgetrennt (Zerlegung von L): Regel und Engine bleiben in B-967.
