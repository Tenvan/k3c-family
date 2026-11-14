# EP1.2 · Tasten-Icons je Eingabegerät in allen Hinweisen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/ep1
- **Abhängig von:** EP1.1
- **Tickets:** B-894
- **Kriterien:** AC-04, AC-05

## Ziel

Eine zentrale Funktion Aktion + Eingabe-Profil → Tasten-Icon; alle Tasten-Hinweise des HUD (Skill-Menü, Beitritt, Bedienhinweis) zeigen die Taste im Profil des jeweiligen Spielers, F1 zeigt Tastenkappen; fehlende Papierschnitt-Icons als PAP-Ticket.

## Kontext

- `native/k3c-game/hud/glyphs.go`: `glyphOf(lang, action, DeviceID)` wählt heute nach `DeviceID.Index` (alte Hälften); Glyphen zeichnet `hud/drawmenu.go` › `drawGlyph` (Kreis für Xbox-Tasten, Kappe für Tasten).
- Das Profil eines Geräts liefert `view.Settings.ProfileOf` (EP1.1); `Frame.Settings` trägt es in jedes Bild.
- `hud/skilltext.go` baut Zeilen und Bedienhinweis des Skill-Menüs aus `k.dev`.
- F1-Übersicht: `devtools/help.go` (UI-Tasten, unabhängig vom Profil).
- Papierschnitt-Icons nur über `assets_*` (Gruppe `icons`); `gen:paper` läuft nicht in der Session (B-742).

## Erlaubte Dateien

- `native/k3c-game/hud/**`
- `native/k3c-game/view/**`
- `native/k3c-game/devtools/**`
- `native/k3c-game/game/**`
- `native/k3c-game/e2e/**`

## Nicht-Ziele

- Papierschnitt-Grafik der Icons selbst (PAP-Ticket).
- Änderung der Belegung (EP1.1).

## Schritte

1. `hud`: `KeyIcon(action, profile)` als einzige Quelle für Glyph-Schlüssel und Asset-Namen; `glyphOf` nimmt das Profil statt des Geräts.
2. Skill-Menü, Bedienhinweis und Beitritts-Hinweis über das Profil aus `Frame.Settings` (Schlüssel der Memos mit Profil).
3. F1-Übersicht: Tastenkappen-Schlüssel für die UI-Tasten.
4. PAP-Ticket für die Papierschnitt-Icons (Liste der Asset-Namen).
5. Tests, `task check:diff`, Commit.

## Fertig, wenn

- [ ] AC-04: Test: für jede Aktion von `glyphActions` und jedes Profil liefert `KeyIcon` einen Schlüssel und eine Beschriftung.
- [ ] AC-05: Test: Profilwechsel in den Einstellungen ändert Skill-Menü-Zeilen und Bedienhinweis im nächsten Aufbau.
- [ ] PAP-Ticket angelegt.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
