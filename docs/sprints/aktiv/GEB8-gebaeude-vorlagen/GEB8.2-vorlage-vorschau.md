# GEB8.2 · Vorlage in der Asset-Vorschau zeigen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb8/2-vorlage-vorschau
- **Abhängig von:** GEB8.1
- **Tickets:** B-441
- **Kriterien:** AC-03

## Ziel

Die Asset-Vorschau (`assetvorschau.html?szene=gebaeude`, Streifen „Neue Gebäude (GEB)“) zeigt unter jedem Gebäude Motiv und Teile seiner Vorlage aus `index.json`.

## Kontext

- **Spec:** Sprint-README GEB8 AC-03.
- `src/tools/assetvorschau.ts` (`renderNeueGebaeude` lädt `grafik/gebaeude/index.json`, `gebaeudeKarten`, `baumReihe` aus `assetvorschauLogic.ts`). Text-Aufbereitung (Motiv, Teile als Liste) als reine Funktion in `assetvorschauLogic.ts` mit Vitest; DOM-Zeichnung in `assetvorschau.ts`. Bäume: Vorlage je Art unter der Baum-Reihe.
- Texte der Seite stehen in der Vorlage selbst (deutsch); keine neuen Übersetzungs-Schlüssel nötig außer einer Überschrift, falls die Seite `t()` nutzt.

## Erlaubte Dateien

- `src/tools/assetvorschau.ts`, `src/tools/assetvorschauLogic.ts`, `src/tools/assetvorschauLogic.test.ts`
- `src/tools/texts.de.ts`, `src/tools/texts.en.ts` (nur eine Überschrift, falls nötig)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Kein Bearbeiten der Vorlage in der Vorschau, keine Änderung an Werkzeug oder Daten, kein Umbau der übrigen Mini-Szenen.

## Schritte

1. `sprint/geb8` auschecken, `Status: in Arbeit`, committen, pushen.
2. Test zuerst: Aufbereitung einer Vorlage (Motiv als Titel, Teile als Liste; fehlende Vorlage → nichts).
3. Funktion umsetzen, in `renderNeueGebaeude` unter jede Karte bzw. die Baum-Reihe hängen.
4. Code-Review der geänderten Dateien, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-03: Vitest in `assetvorschauLogic.test.ts` prüft die Aufbereitung der Vorlage.
- [ ] `task check` grün.

## Prüfen

```bash
task check
```

Keine manuellen Prüfungen in dieser Session; die Sichtprüfung gehört GEB8.3.

## Ergebnis

2026-10-16, Claude (autonom, Ralf im Chat).
- AC-03 umgesetzt: `vorlageZeilen` in `assetvorschauLogic.ts` (Motiv als Titel, Teile, dann Materialien, Stimmung, fein; ohne Vorlage `null`), Vitest-Block „Gebäude-Vorlage (GEB8/AC-03)“ (erst rot, dann grün). `assetvorschau.ts` hängt unter jede 32-px-Karte (Gebäude und einzelne Bäume) ein aufgeklapptes `details` mit der Vorlage, Text nur über `textContent`. Sichtprüfung gehört GEB8.3.
- Checks: `task check` grün. Review (Sonnet-Agent): keine schweren Befunde. Zwei leichte nicht übernommen: fehlende `teile` im Index und doppelte IDs schließt `tools/gebaeude` aus (Pflichtfeld, ID = Dateiname).
