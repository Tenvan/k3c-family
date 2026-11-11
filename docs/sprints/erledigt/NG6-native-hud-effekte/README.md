# NG6 · CLI · Natives Spiel: HUD, Hinweise und Effekte

- **Status:** erledigt
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-799
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Nach NG1 zeichnen `native/k3c-game/hud` und `fx` nur Platzhalter. Vorbild: `src/scenes/hud/`, `hints/`, `effects/`, `kampfAnzeige.ts`, `render/fontRules.ts`, `render/damageRules.ts`. Einzelheiten: B-799.

## Ziel

HUD, Hinweise, Skill-Menü und Effekte zeigen nativ dieselben Informationen wie im Browser. Am Ende sichtbar: Gold, Vorrat, Banner, Radar, Aktions-Hinweise mit Xbox-Glyphen und Schadenszahlen je Zelle.

## Beteiligte und Zielgruppen

Familie, Claude (Umsetzung), 🧑 Ralf (Sichtprüfung in NG9).

## Anforderungen

`B-799 › Anforderungen`.

## Nicht-Ziele

`B-799 › Nicht-Ziele`; Vertrag `view` ändern.

## Regeln und Einschränkungen

Gold nur im HUD; Schrift aus `art` (NG1). Vertrag `view` fest.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/hud/**`, `native/k3c-game/fx/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe
- **Sperre:** –

## Beispiele

`B-799 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-799 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** HUD-Werte und Layout (`B-799/AC-01`).
- **AC-02** Radar, Hinweise, Labels, Kampf-Anzeige (`B-799/AC-02`).
- **AC-03** Aktions-Hinweise, Glyphen, Anleitung, Skill-Menü (`B-799/AC-03`).
- **AC-04** Effekte zu den Ereignissen (`B-799/AC-04`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG6.1 | `NG6.1-hud-grundlage.md` | Umsetzung | autonom | fertig |
| NG6.2 | `NG6.2-radar-hinweise.md` | Umsetzung | autonom | fertig |
| NG6.3 | `NG6.3-aktions-hinweise-skillmenue.md` | Umsetzung | autonom | fertig |
| NG6.4 | `NG6.4-effekte.md` | Umsetzung | autonom | fertig |
| NG6.5 | `NG6.5-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-11 Review (Claude, `k3c-reviewer`): Kriterien laut Ergebnis der Sessions, behobene Befunde siehe Review-Session. Offen: Schriftdatei fehlt (Debug-Schrift); Fenster und Geräte am echten PC ungeprüft. Kein Abnahme-Ticket. Version: keine vorgeschlagen (noch kein spielbarer Stand).
