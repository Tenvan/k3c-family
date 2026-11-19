# B-911 · Tastenkappen und Xbox-Tasten gibt es als Papierschnitt-Icons der Gruppe icons

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU61
- **Projekt:** RSU
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

EP1.2 (B-894) führt alle Tasten-Hinweise über eine zentrale Funktion Aktion + Eingabe-Profil → Glyph (`native/k3c-game/hud/glyphs.go` › `glyphOf`, `IconAsset`). Gezeichnet werden die Glyphen noch selbst (`native/k3c-game/hud/drawmenu.go` › `drawGlyph`: Kreis für A/X/Y, Kappe für Tasten und Schultertasten); Papierschnitt-Icons dazu fehlen. Die Beschriftung der Tastenkappen hängt an der Sprache (`keyLabel(lang, name)`, z. B. „Entf“). Andere HUD-Icons kommen schon aus der Gruppe `icons` (`native/k3c-game/hud/icons.go` › `hudAssets`, `Assets`; `data/paper.json`).

## Ziel

Jeder Glyph-Schlüssel hat ein Papierschnitt-Icon im gleichen Stil wie die übrigen Icons; das HUD zeichnet sie statt der gemalten Glyphen.

## Beteiligte und Zielgruppen

Spielende (Tastatur und Xbox-Pad, je Spieler eigenes Profil); Agent PAP (Icons über `assets_*`, Agent `k3c-grafik` prüft), Agent CLI (HUD, Agent `k3c-hud` prüft); 🧑 nimmt am PC ab.

## Anforderungen

- Icons über `assets_*` in der Gruppe `icons`, eingetragen in `data/paper.json`.
- Xbox-Tasten in Xbox-Farben, je ein Icon `icons/xbox-<name>`: A, X, Y, LB, RB, LT, RT, RS, UP, DOWN, MENU (nur Xbox-Symbole, Plattform-Regel).
- Tastenkappen: zwei leere Kappen `icons/taste-kappe` (ein Zeichen) und `icons/taste-kappe-breit` (längere Beschriftung); die Beschriftung setzt das HUD mit der Spielschrift in der Sprache des Spiels darauf (SPACE, ESC, F1 … F12, E, Q, R, T, Z, K, A, D, ENTER, CONTROLRIGHT, INSERT, DELETE, END, PAGEDOWN, DOWN, LEFT, RIGHT).
- `IconAsset` liefert für jeden Glyph-Schlüssel ein vorhandenes Icon; `hud` meldet die Icons als Asset-Quelle (`Assets`) und zeichnet sie; ohne Grafik bleibt `drawGlyph` der Rückfall.
- Zwei Spieler mit verschiedenen Profilen sehen je ihre Glyphen; keine Allokation je Frame durch die Icons.

## Nicht-Ziele

Neue Belegungen oder weitere Geräte-Familien (B-339 verworfen); je Taste und Sprache ein eigenes Icon.

## Regeln und Einschränkungen

Icons nur über `assets_*` (Entscheidung 005); `gen:paper` und Grafik-Golden nur in der Icon-Session (am Feature, B-742). Das Spiel zeichnet nur.

Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game`; der Go-Test `e2e/glyphs_test.go` hat sein Pendant unter `rust/crates/k3c-game/tests/` (Vorlage nur im Tag `go-referenz-final`, Entscheidung 013).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `public/grafik/icons/**`, `data/paper.json`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/tests/**`
- **Abhängig von:** B-894
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Skill-Menü am Pad → Zeile mit Papierschnitt-LB statt gemaltem Kasten; Profil pfeile, Sprache Deutsch → breite Papier-Kappe mit „Entf“.

## Ausnahme- und Fehlerfälle

Icon fehlt oder Grafik nicht geladen → gemalte Glyphe (`drawGlyph`) wie bisher. Beschriftung länger als die breite Kappe → Schrift wird verkleinert, nie abgeschnitten.

## Akzeptanzkriterien

- **AC-01** Für jeden Schlüssel aus `IconAsset` über alle Aktionen und Profile gibt es ein Icon in `data/paper.json` und `public/grafik/icons/` (Test).
- **AC-02** Ohne Icon zeichnet das HUD die gemalte Glyphe (Test).
- **AC-03** Sichtprüfung durch 🧑 am PC: Skill-Menü und F1-Übersicht mit Tastatur- und Pad-Profil, 2 Spieler.

## Offene Fragen

- Je Taste ein Icon mit eingebrannter Beschriftung oder leere Kappe plus Schrift des Spiels? → Default: zwei leere Kappen plus Schrift, weil die Beschriftung je Sprache wechselt und so 13 statt rund 43 Icons genügen (vorläufig, 🧑 bestätigt; ändert die Namensregel `icons/taste-<name>` aus Revision 1).
- Farben der Xbox-Tasten? → Default: A grün, X blau, Y gelb, Schultertasten und MENU hell, in den Papiertönen des Stils (vorläufig, 🧑 bestätigt bei der Abnahme).

## Notizen

Angelegt in EP1.2 (B-894: „Fehlende Icons als PAP-Ticket“). Revision 2 (2026-11-15): Tastenkappen als leere Kappe plus Schrift statt je Taste ein Icon; Abnahme nur am PC (Entscheidung 009).
