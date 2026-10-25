# GRJ1.1 · Alte Hintergründe mit Lizenz als Git-Tag archiviert

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** grj1/1-archiv-tag
- **Abhängig von:** –
- **Tickets:** B-453
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

- AC-01: umgesetzt. Das annotierte Tag `archiv/hintergruende-vor-geb7` (auf `e0d26d0e^`, gepusht) hält die fünf alten Pack-Hintergründe samt `LICENSE.txt`: blue-cave-background, forest-background, sunnyland-tall-forest-environment, warped-caves-pixel-art-pack, warped-super-grotto-escape-pack.
- Abruf z. B. mit `git show archiv/hintergruende-vor-geb7:public/grafik/forest-background/LICENSE.txt` oder `git archive archiv/hintergruende-vor-geb7 public/grafik/forest-background`.
- Das Repo wächst nicht, die Bilder kommen nicht zurück nach `public/`.
- Geprüft: Tag auf `origin` vorhanden (`git ls-remote`).
