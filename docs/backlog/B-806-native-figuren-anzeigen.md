# B-806 · Das native Spiel zeigt Lebensbalken, Beute-Gold, Monarchen-Punkt und Last wie der Browser-Client

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Funktionsabgleich NG9.1 zeigt: Das native Spiel zeichnet Figuren ohne die Anzeigen des Browser-Clients. Es fehlen Lebensbalken über Truppen, Gegnern und Monarchen (`src/scenes/render/worldRenderer.ts:310`, `:334`, `:390`, Hilfsfunktion `bar` `:97`), der farbige Punkt unter dem Monarchen (`:368`), der Goldkreis bei Gegnern mit geklautem Gold (`:326`, `:333`, `carriedGold`), das Ausblenden freier Monarchen (`GameScene.ts:370-373`, `worldRenderer.ts:212`) und die Last der tragenden Bauern (`:294`, `:308-309`). Die Figuren liegen in `native/k3c-game/actors/`.

## Ziel

Wer am PC spielt, erkennt Lebenspunkte, Beute, Spielerfarbe und Last an den Figuren wie im Browser-Client.

## Beteiligte und Zielgruppen

Spieler am PC und Mini-PC, Claude (Umsetzung), Ralf (Abnahme am Bild).

## Anforderungen

- Lebensbalken über Truppen, Gegnern und Monarchen wie `bar` im Browser.
- Farbiger Punkt unter dem Monarchen in der Spielerfarbe.
- Goldkreis bei Gegnern mit `carriedGold` > 0.
- Freie Monarchen (ohne Spieler) werden nicht gezeichnet; die Information kommt aus `Frame.Seats`.
- Bauern zeigen die getragene Last.
- Kein Zeichnen im `Update`, keine Allokation je Bild; Werte kommen aus dem Frame, die Engine rechnet nichts neu.

## Nicht-Ziele

Neue Optik, neue Werte. Darstellungs-Kleinteile siehe B-811, Menü siehe B-812.

## Regeln und Einschränkungen

Entscheidung 008, Importregeln der Pakete unter `native/k3c-game`, Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Deterministisch: keine Würfel im Client.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/actors/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Truppe mit halber Gesundheit → Balken halb gefüllt über der Figur. Gegner mit geklautem Gold → Goldkreis. Monarch eines Spielers → Punkt in dessen Farbe; Monarch ohne Spieler → unsichtbar.

## Ausnahme- und Fehlerfälle

Volle Gesundheit: Browser-Verhalten übernehmen (laut `bar`), im Test festhalten. Figur ohne Lebenswert im Frame → kein Balken, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Test in `native/k3c-game/actors`: Balkenbreite und Sichtbarkeit je Figurentyp entsprechen den Browser-Regeln aus `worldRenderer.ts`.
- **AC-02** Test: Goldkreis nur bei `carriedGold` > 0; Punkt in der Farbe des Spielers; freier Sitz (`Frame.Seats`) wird nicht gezeichnet.
- **AC-03** Test: Bauer mit Last zeichnet die Last, ohne Last nicht.
- **AC-04** Bildvergleich mit zwei Läufen gleich (wie `TestFlowImages`), Sichtprüfung am PC durch 🧑.

## Offene Fragen

keine

## Notizen

Gefunden im Funktionsabgleich NG9.1 (2026-11-11). Verwandt: B-811 (Kleinteile).
