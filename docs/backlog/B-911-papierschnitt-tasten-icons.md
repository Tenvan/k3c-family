# B-911 · Tastenkappen und Xbox-Tasten gibt es als Papierschnitt-Icons der Gruppe icons

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** EP1
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

EP1.2 (B-894) führt alle Tasten-Hinweise über eine zentrale Funktion Aktion + Eingabe-Profil → Glyph (`native/k3c-game/hud/glyphs.go` › `glyphOf`, `IconAsset`). Gezeichnet werden die Glyphen noch selbst (`hud/drawmenu.go` › `drawGlyph`: Kreis für A/X/Y, Kappe für Tasten und Schultertasten); Papierschnitt-Icons dazu fehlen.

## Ziel

Jeder Glyph-Schlüssel hat ein Papierschnitt-Icon im gleichen Stil wie die übrigen Icons; das HUD zeichnet sie statt der gemalten Glyphen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Icons über `assets_*` in der Gruppe `icons`, Namen nach `IconAsset`: `icons/taste-<name>` und `icons/xbox-<name>`.
- Tastenkappen (Beschriftung generiert): SPACE, ESC, F11, E, Q, R, T, Z, K, A, D, ENTER, CONTROLRIGHT, INSERT, DELETE, END, PAGEDOWN, DOWN, LEFT, RIGHT, dazu F1 … F12 für die F1-Übersicht.
- Xbox-Tasten in Xbox-Farben: A, X, Y, LB, RB, LT, RT, RS, UP, DOWN, MENU (nur Xbox-Symbole, Plattform-Regel).
- `hud` meldet die Icons als Asset-Quelle und zeichnet sie; ohne Grafik bleibt `drawGlyph` der Rückfall.

## Nicht-Ziele

Neue Belegungen oder weitere Geräte-Familien.

## Regeln und Einschränkungen

Icons nur über `assets_*`; `gen:paper` nur am Feature (B-742).

- **Bereich:** `public/grafik/icons/**`, `data/paper.json`, `native/k3c-game/hud/**`
- **Abhängig von:** B-894
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Skill-Menü am Pad → Zeile mit Papierschnitt-LB statt gemaltem Kasten; Profil pfeile → Kappe „Entf“.

## Ausnahme- und Fehlerfälle

Icon fehlt oder Grafik nicht geladen → gemalte Glyphe (`drawGlyph`) wie bisher.

## Akzeptanzkriterien

- **AC-01** Für jeden Schlüssel aus `IconAsset` über alle Aktionen und Profile gibt es ein Icon (Test).
- **AC-02** Ohne Icon zeichnet das HUD die gemalte Glyphe (Test).
- **AC-03** Sichtprüfung durch 🧑 (TV und PC).

## Offene Fragen

keine

## Notizen

Angelegt in EP1.2 (B-894: „Fehlende Icons als PAP-Ticket“).
