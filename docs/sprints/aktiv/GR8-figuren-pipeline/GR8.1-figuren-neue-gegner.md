# GR8.1 · Sechs neue Gegner zeigen eigene Figuren statt Platzhalter

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** gr8/1-figuren-neue-gegner
- **Abhängig von:** –
- **Tickets:** B-329
- **Kriterien:** AC-01

## Ziel

Die sechs neuen Gegner (`lavaSlime`, `ironBeetle`, `fireSpirit`, `crystalSpider`, `shardling`, `crystalGuardian`) zeigen je eine eigene Figur; keine Zeile in `docs/assets/zuordnung-objekte.md` nennt „Platzhalter (B-326)“.

## Kontext

- `data/sprites.json` › `enemies.<id>` verweist auf `sheets.<sheet>` (Frame-Größe, `origin`, `height`, `scale`, `anims`) mit optionaler `tint`; Bilder `public/sprites/<sheet>/<anim>.png` als horizontale Streifen. Credits: `public/sprites/CREDITS.md`; Test `src/tools/credits.test.ts`.
- Aktuell Platzhalter: lavaSlime/shardling=`mushroom`, ironBeetle/crystalSpider=`hell-hound`, fireSpirit=`ghost`, crystalGuardian=`hell-gato`, alle getönt.
- Entscheidung (autonom, Claude, 2026-10-22; Auswahl durch 🧑 GR7.3 offen): ungenutzte Sheets nehmen: lavaSlime=`fire-worm`, fireSpirit=`cemetery-ghost`, shardling=`flying-eye`, crystalGuardian=`skeleton` (Schildträger), jeweils mit passender Tönung. Eisenkäfer und Kristallspinne: eigene, deterministisch erzeugte Pixel-Sheets (neues Sheet `iron-beetle`, `crystal-spider`, CC0, eigene Arbeit), erzeugt von einem kleinen Go-Generator oder Skript (`task gen:...` nur falls nötig; sonst einmalig erzeugt und eingecheckt).
- Sheets ohne `run` (ghost, flying-eye) sind im Client erlaubt (Bestand: `ghost`).

## Erlaubte Dateien

- `data/sprites.json`, `public/sprites/**` (neue Ordner), `public/sprites/CREDITS.md`
- `docs/assets/zuordnung-objekte.md`, `docs/backlog/`, `docs/sprints/`
- Generator für die beiden Sheets: `tools/spritegen/` (neu, eigenes `main.go` im Root-Modul wie `tools/atlas`), Test daneben
- Tests unter `src/tools/` nur, wenn ein bestehender Test Sheets vollständig verlangt

## Nicht-Ziele

Neue Animationen, Werte der Gegner (BR2), Atlas (GR4), Pipeline (GR8.2).

## Schritte

1. Branch `sprint/gr8`, Status `in Arbeit`. Sheets und `sprites.json` lesen.
2. Vier Zuordnungen in `enemies` umstellen, Höhe/`scale` ganzzahlig wählen (GR7).
3. Generator für `iron-beetle` und `crystal-spider` (idle/run/attack, Frame z. B. 32×24), deterministisch, DB32-Farben aus `tools/buildings/db32.go`; Sheets erzeugen, ansehen, nachbessern.
4. Credits, Zuordnung ohne Platzhalter-Vermerk, Tests gezielt.
5. `task check:diff`; Ergebnis, Status `fertig`.

## Fertig, wenn

- [ ] AC-01: `data/sprites.json` weist den sechs Gegnern je eine Figur zu, die kein anderer Gegner mit gleichem Sheet teilt (außer bewusst genannt); `grep "Platzhalter (B-326)" docs/assets/zuordnung-objekte.md` leer.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
