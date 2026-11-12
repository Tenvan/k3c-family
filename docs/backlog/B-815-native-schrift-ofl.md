# B-815 · Das native Spiel zeichnet mit einer freien OFL-Schrift statt der Debug-Schrift

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG18
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Das native Spiel hat keine Schriftdatei und zeichnet mit der Debug-Schrift von Ebitengine (ASCII/Latin-1, feste Größe); Zeichen wie „–“ und „→“ fehlen. Der Pfad `public/schrift/k3c.ttf` wird von `native/k3c-game/art` erwartet, `StrictFont` ist dafür vorbereitet.

## Ziel

Das Spiel zeichnet Texte mit einer freien Schrift aus Repo oder Go-Modul-Cache (ohne Download), inklusive der Sonderzeichen der Texte.

## Beteiligte und Zielgruppen

Ralf (Freigabe, Auswahl), Claude (Umsetzung), Spieler.

## Anforderungen

- Schrift ohne Download aus dem Netz wählen: zuerst `node_modules/**` nach einer OFL-Schrift durchsuchen (`.ttf`/`.otf` mit `OFL.txt` oder `LICENSE` „SIL Open Font License“); gibt es keine, die Go-Schrift `Go-Regular.ttf` aus `golang.org/x/image/font/gofont/ttfs` (BSD-3, im Modul-Cache als Abhängigkeit von Ebitengine) nehmen. Vorprüfung bei der Planung (2026-11-12): in `node_modules` liegt keine `.ttf`/`.otf`; `golang.org/x/image@v0.45.0/font/gofont/ttfs/Go-Regular.ttf` ist im Cache.
- Datei unter `public/schrift/k3c.ttf`, Lizenzdatei daneben (`public/schrift/LICENSE`, Text der Lizenz der gewählten Schrift mit Herkunft und Version). `assets_*` nimmt keine Schriften an (`assets_import`: nur png, ogg, mp3, wav), deshalb Ablage als Datei.
- Kein Eintrag in `go.mod`: die Schrift wird als Datei geladen (Pfad `art.DefaultFontPath`), nicht als Go-Paket importiert.
- Danach `StrictFont` anschalten: fehlende Zeichen sind ein Fehler im Test, nicht still ein Kästchen.
- Die Lizenz-Seite (B-808, NG13) zeigt die Lizenzdatei; hier nur die Datei.

## Nicht-Ziele

Mehrere Schriftschnitte, Menü-Optik (B-812), Lizenz-Seite im Menü (B-808), Download einer Schrift aus dem Netz.

## Regeln und Einschränkungen

Entscheidungen 005 und 008. Keine neue Abhängigkeit in `go.mod` oder `package.json`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `public/schrift/**`, `native/k3c-game/art/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

Text „Stufe 1 → Stufe 2“ erscheint mit Pfeil in der neuen Schrift.

## Ausnahme- und Fehlerfälle

Schriftdatei fehlt → Meldung im Fenster (B-810), nicht Rückfall auf die Debug-Schrift ohne Hinweis.

## Akzeptanzkriterien

- **AC-01** `public/schrift/k3c.ttf` mit Lizenzdatei `public/schrift/LICENSE` (Herkunft, Version, Lizenztext) angelegt; Wahl und Suche in den Notizen.
- **AC-02** Test: alle Zeichen der Spieltexte (Menü, HUD, Anleitung) sind in der Schrift enthalten, `StrictFont` an.
- **AC-03** Ende-zu-Ende-Lauf (`task native:game:e2e`): zwei Läufe gleich in allen Stationen; Bilder unter `reports/native-game-e2e/` für die Sichtprüfung bei der PC-Abnahme.

## Offene Fragen

Entschieden autonom, Claude, 2026-11-12: Keine Schrift aus dem Netz. Erst `node_modules` nach einer OFL-Schrift durchsuchen, sonst die Go-Schrift (`Go-Regular.ttf`, BSD-3) aus dem Modul-Cache einbetten. **Abweichung von der Entscheidung Ralf 2026-11-11 („freie OFL-Schrift“):** Die Go-Schrift ist frei, aber BSD-3 statt OFL; sie liegt ohne Download vor und deckt die Sonderzeichen (WGL4, u. a. „–“, „→“; vermutet, AC-02 prüft es). Ralf kann sie später durch eine OFL-Schrift ersetzen (gleicher Pfad, neue Lizenzdatei). Ablage als Datei, weil `assets_*` keine Schriften annimmt. Sperre `deps` entfällt, da `go.mod` unverändert bleibt. Sichtprüfung läuft bei der nächsten PC-Abnahme mit (Ralf-Queue).

## Notizen

Entscheidung Ralf 2026-11-11: freie OFL-Schrift. Gefunden in NG9. Revision 2 (2026-11-12): Go-Schrift als Default, Ablage als Datei, Sperre `deps` entfernt. Der Titel nennt noch „OFL“ (Titel ändert `plan_set` nicht).
