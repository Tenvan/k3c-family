# PG1.1 · Echte Spielbilder aufnehmen und einbinden

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** live
- **Branch:** pg1/1-bilder-einbinden
- **Abhängig von:** –
- **Tickets:** B-184
- **Kriterien:** AC-01

## Ziel

Ein bis zwei Sätze: welches Ergebnis nach dieser Session auf `develop` liegt.

## Kontext

Alles, was ein Agent **ohne Vorwissen** braucht: Stand des Codes, relevante Dateien mit Pfad, Entscheidungen
(`docs/decisions/…`), Fallstricke. Keine Verweise auf Chat-Verläufe.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `pfad/…`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Konkrete, nummerierte Schritte in sinnvoller Reihenfolge.

## Fertig, wenn

- [ ] AC-01: Jeder Punkt ist prüfbar: ein Befehl mit erwartetem Ergebnis, ein Test, eine Datei, eine Beobachtung.
- [ ] Punkte ohne Kriterium sind technische Voraussetzungen (z. B. CI grün).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01: umgesetzt. `site/index.html` hat den neuen Abschnitt „Aus dem Spiel“ mit drei echten Bildern (1280 × 720 JPEG, je ca. 110–160 KB) unter `site/img/spiel/`:
  - Split-Screen bei Nacht (Spielstand `familie`, 2 Spieler)
  - Hub bei Tag (Voll-Ausbau-Stand `voll-praesentation`)
  - Nachtwelle an Turm und Mauer
- Aufgenommen mit `game.renderer.snapshot` im Browser-Pane und über `/api/report` abgelegt; die Bilddaten gingen so nicht durch den Chat.
- Geprüft: `task check` grün, alle Bildpfade vorhanden, Bilder angesehen. Das Layout der Seite selbst konnte Claude im Pane nicht ansehen (statische Vorschau ohne relative Bilder); Ralf sieht die Seite nach dem Merge auf Pages.
