# WZ7.2 · Seite Welt-Betrachter mit Chunk-Vorschau im Studio

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz7/2-seite-chunk-vorschau
- **Abhängig von:** –
- **Tickets:** B-460
- **Kriterien:** AC-01, AC-02, AC-08

## Ziel

Im Asset-Studio gibt es die Seite Welt-Betrachter mit der Chunk-Vorschau im Umfang von `leveltest.html`, ohne Spielserver, beim Öffnen maximiert und verkleinerbar (AC-01, AC-02).

## Kontext

Sprint WZ7 (`docs/sprints/geplant/WZ7-welt-betrachter/README.md`), Ticket `docs/backlog/B-460-welt-betrachter-live-vorschau.md` (Spec freigegeben 2026-10-17). Maßgeblich sind die Anforderungen, Beispiele und Ausnahmefälle dort.

- Asset-Studio aus WZ6: `tools/k3c-studio` (Wails, Frontend mit Aliasen `@game`/`@data`, Modul `k3c` über `replace` wie `tools/k3c-dev`). Gestaltung der Seiten Grafik/Sound dort als Vorbild.
- Heute: `leveltest.html` mit `src/tools/leveltest.ts`, Logik in `levelView.ts`/`levelApi.ts` (mit Tests); Level kommt per `GET /api/level` vom Spielserver. Neu: Go-Binding auf `engine/level` (Quelle als Wert, in WZ7 das Biom; Schnittstelle kann später Insel + Stufe aufnehmen).
- Logik und Tests ziehen ins Studio-Frontend um (wie `assetvorschau` in WZ6); `leveltest.html` bleibt bis WZ7.5 bestehen.
- Fenster- und Browserprüfungen nur mit Freigabe von 🧑 für den Lauf.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `tools/k3c-studio/**` (Go-Binding, Frontend-Seite, Logik, Tests, Codemap)
- `src/tools/levelView.ts`, `src/tools/levelApi.ts` und deren Tests (nur lesen/kopieren; Entfernen erst WZ7.5)
- diese Session-Datei

## Nicht-Ziele

Kein Entfernen von `leveltest.html` (WZ7.5). Keine Live-Vorschau, kein Phaser (WZ7.3). Keine Änderung unter `src/scenes/`, `src/online/`, `src/model/`, `public/`, `data/`, `engine/`. Keine Chunk-Planung (B-461).

## Schritte

1. Go-Binding auf `engine/level`: Layout (Chunks) zu Seed und Quelle (Wert: Biom), Warnungen der Spielbarkeits-Prüfung; Seed-Prüfung wie `/api/level`.
2. Logik und Tests aus `levelView.ts`/`levelApi.ts` ins Studio-Frontend verschieben bzw. kopieren und auf das Binding umstellen.
3. Seite Welt-Betrachter mit Chunk-Streifen im Umfang von `leveltest.html`; beim Öffnen maximiert, per Schalter verkleinerbar (Live-Vorschau darunter folgt in WZ7.3).
4. Vitest und Go-Tests; Codemap nachziehen.

## Fertig, wenn

- [ ] AC-01: Chunk-Vorschau zeigt zu Seed und Biom dasselbe wie `leveltest.html`, ohne Spielserver (Tests der umgezogenen Logik, Go-Test des Bindings).
- [ ] AC-02: Chunk-Vorschau ist beim Öffnen maximiert und verkleinerbar (Frontend-Test).
- [ ] AC-08: `check_run` `task:check`, `go:test` und `go:lint` grün.

## Prüfen

`task check` und `task check:go`; über k3c-dev `check_run` (mit `checkout`). Fenster-Beobachtung nur mit Freigabe von 🧑.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
