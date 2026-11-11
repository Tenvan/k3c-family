# B-799 · Das native Spiel zeigt HUD, Hinweise, Skill-Menü und Effekte wie der Browser-Client

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG6
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Der Browser-Client zeigt HUD, Hinweise und Effekte in `src/scenes/hud/` (Gold, Vorrat, Banner, Radar, Pause, Speicher- und Beitritts-Hinweis, Skill-Menü), `src/scenes/hints/` (Aktions-Hinweise, Glyphen, Anleitung), `src/scenes/effects/` (Skills, Status, Effekte), `src/scenes/kampfAnzeige.ts` und `render/damageRules.ts`; Schriftrollen in `render/fontRules.ts`.

## Ziel

Pakete `native/k3c-game/hud` und `native/k3c-game/fx` zeigen dieselben Informationen je Zelle wie der Browser-Client.

## Beteiligte und Zielgruppen

Familie, Claude (Umsetzung), 🧑 Ralf (Sichtprüfung NG9).

## Anforderungen

- HUD je Zelle: Gold, Vorrat, HP, Banner-Warteschlange, Radar, Pause- und Speicher-Hinweis, Beitritts-Hinweis, Bürger- und Wirtschafts-Labels, Kampf-Anzeige.
- Aktions-Hinweise mit Xbox-Glyphen, Anleitung, Skill-Menü.
- Effekte: Schadenszahlen, Skill-Wirkungen, Status an Figuren.
- Schriftgrößen und Kontur nach `fontRules.ts`; Rechnung (Layout, Texte, Warteschlangen) als reine Funktionen mit Tests.

## Nicht-Ziele

Diagnose- und Debug-Anzeige (außer FPS aus NG1); Menüs (NG7); neue Texte oder Übersetzungen.

## Regeln und Einschränkungen

Gold nur im HUD (`goldOnlyInHud.test.ts`); Textregeln wie `textRule.test.ts`. Schrift aus `art` (NG1). Vertrag `view` unverändert.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `native/k3c-game/hud/**`, `native/k3c-game/fx/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Gegner-Welle naht → Banner oben in der Zelle, Radar zeigt Richtung; Monarch trifft einen Gegner → Schadenszahl steigt auf.

## Ausnahme- und Fehlerfälle

Viele Banner gleichzeitig → Warteschlange wie `bannerQueue.ts`. Kleine Zelle (zwei Spieler) → Mindestgrößen aus `fontRules.ts`.

## Akzeptanzkriterien

- **AC-01** HUD-Werte und Layout je Zelle: Tests nach `hudLayout.ts`, `fontRules.ts` und `bannerQueue.ts` grün.
- **AC-02** Radar, Pause-, Speicher- und Beitritts-Hinweis, Labels und Kampf-Anzeige folgen dem Frame (Tests).
- **AC-03** Aktions-Hinweise, Glyphen, Anleitung und Skill-Menü: Tests nach `actionHints.ts`, `guideHints.ts` und `skillMenuLogic.ts` grün.
- **AC-04** Effekte: Schaden, Skill-Wirkung und Status erscheinen zu den Ereignissen des Frames (Tests nach `effectRules.ts` und `damageRules.ts`).

## Offene Fragen

keine

## Notizen

Angelegt 2026-11-11 bei der Planung der Umsetzung (NG6).
