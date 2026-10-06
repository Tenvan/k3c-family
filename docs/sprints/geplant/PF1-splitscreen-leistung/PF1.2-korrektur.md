# PF1.2 · Hauptlast des Split-Screens senken

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** pf1/2-korrektur
- **Abhängig von:** PF1.1
- **Tickets:** B-194
- **Kriterien:** AC-03, AC-04

## Ziel

Die in PF1.1 gemessene Hauptlast ist behoben oder gesenkt; die Frame-Zeit mit zwei Kameras ist im Browser-Pane vorher und nachher belegt.

## Kontext

- Grundlage sind die Messwerte und die vermutete Ursache in B-194 › Notizen (aus PF1.1). Ohne diese Werte nicht beginnen.
- Kameras und Nacht-Filter: `src/scenes/GameScene.ts` › `layoutCameras()`, `aimAtStage()`, `addNightFx()`; Layer je Kamera über `showOnly()` in `src/scenes/stageView.ts`; Zeichnen in `src/scenes/worldRenderer.ts`.
- Liegt die Hauptlast im Netz oder Server (Snapshot-Takt, Puffer), gehört die Korrektur nicht in CLI: dann Ticket anlegen (Domäne SRV bzw. N2-Nachfolger), hier nur die Messwerte im Ergebnis.
- Fallstrick: `src/scenes/GameScene.ts` hat schon 403 Zeilen. Eine Änderung dort darf die Datei nicht wachsen lassen; Logik als reine Funktion in eine eigene Datei unter `src/scenes/` mit Test.
- Nacht im Canvas-Modus bleibt hell (`addNightFx`); eine Lösung darf das Verhalten bei Tag und Nacht für beide Spieler nicht ändern (2 Spieler gleichzeitig, `CLAUDE.md` › Regeln).
- Ausnahmefall laut Sprint: Ziel nicht erreichbar → Messwerte und Ticket, angenommener Wert bleibt.

## Erlaubte Dateien

- `src/scenes/`
- `docs/backlog/B-194-*.md`, `docs/sprints/`

## Nicht-Ziele

Messung an der Xbox (PF1.4), Netz-Latenz (N2), Atlas (GR4), Server-Leistung (B-042), neue Spielregeln.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. Frame-Zeit mit zwei Kameras (Tag und Nacht) im Browser-Pane wie in PF1.1 messen (Vorher-Wert).
3. Die Hauptlast aus B-194 › Notizen gezielt senken; neue Logik als reine Funktion mit Vitest-Test.
4. Nachher-Wert unter denselben Bedingungen messen; beide Werte ins Ergebnis und in B-194 › Notizen.
5. Prüfen, dass beide Spieler ihre Stufe, Kamera-Folge und Nacht-Abdunklung wie vorher sehen.

## Fertig, wenn

- [ ] AC-03: Ergebnis nennt Frame-Zeit mit zwei Kameras vorher und nachher; nachher ist sie niedriger, oder ein Ticket begründet, warum nicht.
- [ ] AC-04: `task check` grün; keine geänderte Datei über 400 Zeilen, keine Funktion über 60.

## Prüfen

```bash
task check
```

## Ergebnis

–
