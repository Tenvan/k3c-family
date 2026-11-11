# B-815 · Das native Spiel zeichnet mit einer freien OFL-Schrift statt der Debug-Schrift

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das native Spiel hat keine Schriftdatei und zeichnet mit der Debug-Schrift von Ebitengine (ASCII/Latin-1, feste Größe); Zeichen wie „–“ und „→“ fehlen. Der Pfad `public/schrift/k3c.ttf` wird von `native/k3c-game/art` erwartet, `StrictFont` ist dafür vorbereitet.

## Ziel

Das Spiel zeichnet Texte mit einer freien OFL-Schrift, inklusive der Sonderzeichen der Texte.

## Beteiligte und Zielgruppen

Ralf (Freigabe, Auswahl), Claude (Umsetzung), Spieler.

## Anforderungen

- Freie OFL-Schrift über `assets_*` einbinden, Lizenz-Eintrag (Entscheidung Ralf 2026-11-11).
- Datei unter `public/schrift/k3c.ttf`.
- Danach `StrictFont` anschalten: fehlende Zeichen sind ein Fehler im Test, nicht still ein Kästchen.
- Lizenz erscheint in der Lizenz-Seite (B-808).

## Nicht-Ziele

Mehrere Schriftschnitte, Menü-Optik (B-812).

## Regeln und Einschränkungen

Entscheidungen 005 und 008. Assets nur über MCP `assets_*`. Neue Abhängigkeit nur mit Freigabe.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `public/schrift/**`, `native/k3c-game/art/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** deps

## Beispiele

Text „Stufe 1 → Stufe 2“ erscheint mit Pfeil in der OFL-Schrift.

## Ausnahme- und Fehlerfälle

Schriftdatei fehlt → Meldung im Fenster (B-810), nicht Rückfall auf die Debug-Schrift ohne Hinweis.

## Akzeptanzkriterien

- **AC-01** Schriftdatei mit OFL-Lizenz-Eintrag über `assets_*` angelegt.
- **AC-02** Test: alle Zeichen der Spieltexte (Menü, HUD, Anleitung) sind in der Schrift enthalten, `StrictFont` an.
- **AC-03** Ende-zu-Ende-Lauf (`task native:game:e2e`) bleibt gleich in allen Stationen; Sichtprüfung 🧑.

## Offene Fragen

Welche OFL-Schrift? Entscheidet Ralf bei der Freigabe.

## Notizen

Entscheidung Ralf 2026-11-11: freie OFL-Schrift. Gefunden in NG9.
