# NT1.1 · Flakige Tests: TestRestore, TestGleicherSeedGleicheEingaben, TestTickReiheJeRaum

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** nt1/1-flakige-tests
- **Abhängig von:** –
- **Tickets:** B-274, B-284, B-286
- **Kriterien:** AC-01, AC-02, AC-03

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

- AC-01 bis AC-03: umgesetzt.
  - `TestRestore` (B-274): Ursache im Code. `store()` behandelte einen Lesefehler als „kein Stand“ und überschrieb ohne Sicherung. Jetzt liest es begrenzt erneut (6 × 20 ms) und bricht sonst ohne Überschreiben ab. Die Ursache ist vermutet: Sie passt zum Fehlerbild, ließ sich aber nicht nachstellen.
  - `TestTickReiheJeRaum` (B-286): Der Bot schickte nach `leave` noch eine Eingabe, das ergab `bad_request`. Jetzt schickt er nach `leave` nichts mehr.
  - `TestGleicherSeedGleicheEingaben` (B-284): Statt einer festen Wanduhr von 2 s läuft der Test bis 30 Eingaben je Bot (höchstens 30 s).
- Geprüft unter Last (parallel `go test -count=3 -p 8 ./engine/...`): TestRestore `-count=200`, die anderen beiden `-count=20`, alle grün. Review (Opus) ohne schwere Befunde.
