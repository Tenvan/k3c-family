# GEB7.6 · Review und Abnahme

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb7/6-review
- **Abhängig von:** GEB7.5
- **Tickets:** B-435
- **Kriterien:** AC-08

## Ziel

Der Diff von GEB7 ist geprüft, der eine PR des Sprints ist offen; die Abnahme der Hintergründe (AC-08) liegt bei 🧑.

## Kontext

- Leichter Review nach `docs/arbeitsweise.md`: nur schwere Befunde im Diff, getrennter Reviewer; besonders: ältere PNGs unverändert (nur `A` für `hg-*`), gelöschte Grafik nirgends mehr genutzt (auch Fixtures und Mocks unter `tools/k3c-dev`, `site/`), `BACKGROUNDS` deckt alle fünf Biome, Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen, kein `Math.random()`.
- **Sichtprüfung (AC-08):** gehört 🧑. Ein Agent prüft im Browser nur, wenn die Session sie nennt und 🧑 sie für diesen Lauf freigegeben hat; hier gilt: Claude nimmt **nicht** selbst ab. Vorschau `assetvorschau.html?szene=gebaeude` (Streifen „Hintergründe“), im Spiel je Biom Tiefe 0 bis 4 und der Fluss unter dem Hub; Vite und Spielserver über `svc_start`, Adressen über `get_urls`.
- **Abweichung vom Plan (autonomer Lauf, Spec Revision 2):** Das Wald-Gate (🧑 prüft den Wald vor GEB7.2/7.3) ist entfallen; 🧑 prüft **alle fünf Biome** gemeinsam und gibt Motive, Farben und Dichte frei oder lehnt einzelne Schichten ab (Nachbesserung = Farbtausch in der JSON, kein Code). Zusätzlich offen für 🧑 nach Abschluss: Antworten auf die Fragen 1 bis 9 der Spec (Annahmen), besonders Frage 5 (Pages-Seite umgestellt statt unverändert) und Frage 7 (nur Rechtecke genügen?).

## Erlaubte Dateien

- Dateien des Sprint-Diffs, nur für schwere Befunde oder von 🧑 verlangte Nachbesserungen
- `docs/sprints/`, `docs/backlog/`, `docs/projekte/`, `docs/roadmap.md`

## Nicht-Ziele

Stil-Nacharbeit ohne Auftrag von 🧑, neue Biome, Animationen, Skalierungsfaktor am TV (GR7.4), Paletten-Angleich der Biome (eigenes REG-Ticket).

## Schritte

1. `Status: in Arbeit`; `task check` und `task check:go` grün (`check_run`, mit `checkout`).
2. `git fetch && git diff origin/develop...HEAD` lesen (getrennter Reviewer), Befunde in den Domänen des Sprints beheben, sonst Ticket.
3. Nachweis je Kriterium AC-01 bis AC-07 aus den Session-Ergebnissen GEB7.1–GEB7.5; AC-08 bleibt `offen für 🧑` (neues Ticket zur Abnahme anlegen, wie B-449 bei GEB5).
4. Abschnitt „Abnahme“ der Sprint-README (≤ 5 Zeilen) und `Version: v… vorgeschlagen (Grund)` (Minor: Wirkung im Spiel).
5. Fahrplan; ist nur AC-08 offen, bleibt der Sprint `aktiv` (Hardware entkoppelt: Abnahme durch 🧑), sonst nach `erledigt/`. `git fetch`, `git merge origin/develop`, push, **PR** `sprint/geb7` → `develop` öffnen (Vorlage, eine Zeile je Session), Auto-Merge nicht von Hand erzwingen, wenn die Abnahme noch offen ist; kein Merge durch den Agenten ohne Auftrag.

## Fertig, wenn

- [x] AC-08: Abnahme durch 🧑 steht im Ergebnis oder ist als offen für 🧑 geführt (Ticket).
- [x] AC-01 bis AC-07: Nachweise aus GEB7.1–GEB7.5 geprüft; alle Kriterien mit Nachweis im Ergebnis (`umgesetzt`, `geprüft`, `verschoben` mit Ticket); AC-08 als offen für 🧑 geführt.
- [x] `task check` und `task check:go` grün; PR offen.

## Prüfen

```bash
task check
task check:go
```

Manuelle Prüfung im Browser-Pane nur mit Freigabe von 🧑 für diesen Lauf.

## Ergebnis

- **Prüfen:** `check_run` `task:check` (23 s) und `task:check:go` (3 min 33 s) im Worktree grün, Stand `develop` `e29dcc8c` (GEB7.1–7.5 als Squash `e0d26d0e`, PR #324, GEB6 `e29dcc8c` schon enthalten).
- **Review:** `git diff e0d26d0e^ e0d26d0e` von einem vom Schreiber getrennten Reviewer (Sonnet) gelesen: **keine schweren Befunde**. Belegt: 30 × `A` nur `hg-*` (15 Schichten × 16/32 px), kein `M` an PNGs, 36 × `D` nur Pack-Grafik; kein Rest der fünf gelöschten Packs oder von `bg:forest-*`/`bg:cave`/`bg:mine-back` in `src/`, `data/`, `tests/`, `tools/` (inkl. k3c-dev), `site/`, `public/`, Root-`*.html` (Treffer nur in historischer Doku); `BACKGROUNDS` deckt alle fünf Biome mit je drei Schichten, Zoom 1, Scroll aufsteigend; Dateien ≤ 400, Funktionen ≤ 60 Zeilen, kein `Math.random`/`math/rand`.
- **AC-01 geprüft:** GEB7.1–7.3 (15 JSONs, 16/32 px, byte-gleich, ältere PNGs md5-gleich), Regeldatei § 1/§ 5/§ 7/§ 10. **AC-02 geprüft:** `TestKachelKlassen` (Naht, oben transparent, unten deckend), Generator-Tests. **AC-03 geprüft (Logik):** `hintergrundStreifen` mit Vitest; Sicht gehört 🧑. **AC-04 geprüft:** `TestHintergrundStufen` (Luma fern < mitte < nah) und Palette DB32 über `decodePNG` auf `Paletted` und benannte Farben aus `db32.go`; ein ausdrücklicher DB32-Test fehlt (kein schwerer Befund). **AC-05 geprüft:** `worldSprites.test.ts` (fünf Biome, `null` für unbekannte, Zoom, Scroll, Register `bg:<biom>:<n>`), `zuordnung.test.ts`. **AC-06 geprüft:** `git grep` vor und nachher (GEB7.5), Reviewer-Gegenprobe ohne Reste. **AC-07 geprüft:** `site/index.html` verweist nur auf vorhandene `hg-*`; `task pages` baut (GEB7.4).
- **AC-08 offen für 🧑:** Vorschau, Spiel und Pages-Seite hat kein Mensch gesehen; Claude hat nicht abgenommen und keine Sichtprüfung gemacht. Ticket **B-452** (Abnahme, Annahmen 1–9, besonders Frage 5 und 7).
- **Randbeobachtungen (kein Befund):** Kommentar `assetvorschauLogic.ts:124` nennt noch „Wald-Hintergrund ×3“; `B-435` Notizen (Zeile ~101) nennen das alte Schema `bg:forest|cave|mine`. Abweichung: GEB7 liegt in `geplant/`, bis GEB6 `erledigt` ist (ein aktiver Sprint je Projekt). Neue Tickets: B-452.
