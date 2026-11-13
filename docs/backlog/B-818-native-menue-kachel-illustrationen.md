# B-818 · Hauptmenü und Pause-Menü des nativen Spiels zeigen Kacheln mit Illustrationen

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** NG19
- **Projekt:** NAT
- **Erstellt:** 2026-11-12
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** Ralf, 2026-11-13 (Dialog „weiter mit B-818“)

## Ausgangslage

Abgespalten aus B-812 (2026-11-12): B-812 liefert nur Papier-Hintergrund und Karten im Papierschnitt-Stil. Die Illustrationen der Kacheln des Browser-Clients (`src/scenes/pauseHub.ts:62-82`, `src/scenes/hubArt.ts`) fehlen im nativen Spiel weiter.

## Ziel

Haupt- und Pause-Menü des nativen Spiels zeigen je Eintrag eine Kachel mit Illustration wie der Browser-Client.

## Beteiligte und Zielgruppen

Spieler, Ralf (Geschmack, Freigabe der Assets), Claude (Umsetzung).

## Anforderungen

- Kacheln mit Illustration im Hauptmenü und im Pause-Menü auf den Karten aus B-812.
- Illustrationen nur über `assets_*` (Entscheidung 005), Zuordnung Eintrag → Bild an einer Stelle.
- Zerlegung vor der Freigabe prüfen (Kacheln Hauptmenü, Kacheln Pause), falls Größe M überschritten wird.

## Nicht-Ziele

Papier-Hintergrund und Karten (B-812), neue Menüseiten (B-807, B-808, B-809).

## Regeln und Einschränkungen

Entscheidungen 005 und 008. Import-Regeln der Pakete unter `native/k3c-game` (`game/imports_test.go`).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/menu/**`
- **Abhängig von:** B-812
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Hauptmenü: „Weiter“ zeigt eine Kachel mit Bild auf der Papierkarte.

## Ausnahme- und Fehlerfälle

Bild fehlt → Karte mit Text, kein Absturz, Log-Zeile mit Pfad.

## Akzeptanzkriterien

- **AC-01** Test in `menu`: Jeder Eintrag von Haupt- und Pause-Menü hat eine zugeordnete Illustration aus vorhandener Spielgrafik; fehlendes Bild liefert die Text-Karte.
- **AC-02** Die Illustrationen stammen aus vorhandenen Assets (`public/grafik`, Entscheidung 005); neue Assets nur über `assets_*`.
- **AC-03** Sichtprüfung am PC durch 🧑.

## Offene Fragen

keine (entschieden 2026-11-13, Dialog Ralf „weiter mit B-818“, Defaults durch Claude): Illustrationen aus der vorhandenen Spielgrafik wie `src/scenes/hubArtData.ts` (Monarch, Portal, Truppen, Truhe, Ausgang …), keine neuen Assets; Haupt- und Pause-Menü in einer Session.

## Notizen

Abgespalten aus B-812 am 2026-11-12 (autonome Planung der Welle 4, Claude).
