# B-1000 · Gatter und Welt-Reittiere sind im Spiel sichtbar, der Monarch reitet sein gekauftes Tier

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** SI4
- **Projekt:** KMP
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Teil von B-970 (Ideenrunde k3c-07, 2026-11-15): Die Engine kennt nach SI4.3 Gatter und Reittier je Monarch. Das Spiel zeichnet heute für jeden Monarchen das Pferd (`reittiere/pferd`, `data/paper.json`); in `public/grafik/reittiere/` liegen weitere Tiere (unter anderem `rappe`, `schimmel`), ein Kamel und ein Gatter fehlen.

## Ziel

Das Gatter mit dem Welt-Reittier ist auf der Insel zu finden, und jeder Monarch reitet sichtbar sein eigenes Tier.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions (SI4.2 Assets, SI4.4 Anzeige), Fachprüfung durch `k3c-grafik`. Abnahme am TV: 🧑.

## Anforderungen

- Anforderungen aus `B-970 › Anforderungen` gelten; hier nur Bild und Anzeige.
- Assets als Papierschnitt, nur über `assets_*`: `kamel` (Gruppe `reittiere`, gleiche Posen wie `pferd`) und `gatter` (Gruppe `objekte`). Das Sengoku-Pferd nutzt das vorhandene `rappe`.
- Am Gatter steht das Welt-Reittier, solange mindestens ein gesteuerter Monarch es noch nicht hat; die Zahlstellen-Anzeige zeigt den Preis.
- Jeder Monarch wird mit seinem Reittier aus dem Engine-Zustand gezeichnet; zwei Monarchen können verschiedene Tiere reiten.
- Split-Screen: beide Hälften zeigen beide Monarchen mit dem richtigen Tier.

## Nicht-Ziele

Rentier (kommt mit der Polar-Welt), Umstiegs-Animation, neue Sounds, Reittier-Anzeige im HUD, Regeländerungen (B-970).

## Regeln und Einschränkungen

- **Bereich:** `public/grafik/reittiere/**`, `public/grafik/objekte/**`, `data/paper.json`, `native/k3c-game/**`
- **Abhängig von:** B-970
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –
- Zwei Sessions: SI4.2 Assets (PAP, vor der Engine möglich), SI4.4 Anzeige (CLI). Grafik-Golden und `gen:paper` laufen im Abnahme-Ticket des Sprints (B-742).

## Beispiele

- Nil: Am Gatter steht das Kamel. Monarch 1 kauft und reitet das Kamel, Monarch 2 weiter das Pferd; das Kamel steht weiter am Gatter, bis auch Monarch 2 kauft.
- Welt `standard`: kein Gatter, beide reiten das Pferd.

## Ausnahme- und Fehlerfälle

- Katalog-ID ohne Zuordnung in `data/paper.json` → Test schlägt fehl.
- Alle gesteuerten Monarchen haben das Tier → das Gatter steht leer.

## Akzeptanzkriterien

- **AC-01** Die Assets `kamel` (Gruppe `reittiere`) und `gatter` (Gruppe `objekte`) sind über `assets_*` angelegt; `data/paper.json` ordnet `camel`, `sengokuHorse` (auf `rappe`) und das Gatter zu; `k3c-grafik` meldet keinen schweren Befund.
- **AC-02** Go-Test in `native/k3c-game`: Jeder Monarch wird mit dem Reittier aus seinem Zustand gezeichnet; zwei Monarchen mit verschiedenen Tieren.
- **AC-03** Go-Test: Gatter mit Tier, solange ein gesteuerter Monarch es nicht hat, sonst leer.
- **AC-04** Beobachtung am TV (🧑): Kauf am Gatter, Monarch reitet danach das neue Tier, auch nach dem Inselwechsel; beide Hälften stimmen.
- **AC-05** `check_run task:check:diff` grün.

## Offene Fragen

Mit Default entschieden (vorläufig, 🧑 bestätigt):

- Welches Bild hat das Sengoku-Pferd? → Das vorhandene `rappe`; kein neues Asset.
- Umstiegs-Animation? → Nein, das Tier wechselt im selben Bild.

## Notizen

Am 2026-11-15 aus B-970 abgetrennt (Zerlegung von L): Regel und Engine bleiben in B-970.
