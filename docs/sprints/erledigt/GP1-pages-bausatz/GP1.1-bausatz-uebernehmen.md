# GP1.1 · Entwurf Bausatz nach site/ übernehmen, ohne Script, Lizenz-Links prüfen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Lane:** –
- **Modell:** –
- **Umgebung:** ?
- **Branch:** gp1/1-bausatz-uebernehmen
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Bausatz-Entwurf aus `.work/site-vorschlaege/c-bausatz` nach `site/` übernehmen und prüfen.

## Kontext

Alles, was ein Agent **ohne Vorwissen** braucht: Stand des Codes, relevante Dateien mit Pfad, Entscheidungen
(`docs/decisions/…`), Fallstricke. Keine Verweise auf Chat-Verläufe.

## Erlaubte Dateien

- `site/**`
- `tests/pagesSite.test.ts`

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
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: `tests/pagesSite.test.ts` 4/4 grün; das Script der Drehscheibe ist durch CSS (Kontrollkästchen + `:has`) ersetzt; der Test lässt eingebettete `data:`-Bilder zu (keine Datei, keine Fremdadresse).
- AC-02 geprüft: kein „Open Source“ (Suche ohne Groß-/Kleinschreibung); Links LICENSE, LICENSE-ASSETS (statt der mit WR1 entfernten lizenzen.html) und PolyForm liefern 200.
- AC-03 geprüft: lokaler Server, Desktop und 375 px ohne Querscrollen, 0 kaputte Bilder, Drehscheibe schaltet Tag/Nacht hin und zurück; impeccable-Detektor: nur die zwei bewussten Befunde (Matten-Gitter, Kontrast-Fehlmeldung Drehscheibe).
- Schriften: `site/fonts/README.md` nennt die OFL-Lizenzen.
- Offen: Sichtprüfung durch 🧑 (B-908/AC-04).
