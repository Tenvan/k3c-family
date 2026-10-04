# SO4.3 · Ducking bei Warnungen, Musik-Dateien und Credits

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** so4/3-ducking-dateien-credits
- **Abhängig von:** SO4.2, SO2.3
- **Tickets:** B-168
- **Kriterien:** AC-03, AC-04, AC-06

## Ziel

Warn-Sounds senken die Musik hörbar ab und stellen sie danach wieder her; die gewählten Stücke liegen unter `public/audio/` mit Credit-Eintrag, nicht gewählte Kandidaten sind entfernt, und CC-BY-Stücke sind auf der Credits-Seite genannt, soweit diese Audio kennt.

## Kontext

- **Ducking (Beschluss Q16: „Ducking bei Warnungen“):** Warnungen sind laut SO1.3 `dusk` (Nacht naht), `wave` und `castleFallen`; ihre Sounds und das Event-Mapping liefert SO2.3 (`src/audio/`). Beim Auslösen eines Warn-Sounds fällt der Pegel des Busses „Musik“ in kurzer Rampe auf einen Bruchteil und steigt nach dem Warn-Sound in einer Rampe zurück. Werte (Absenkung, Anstieg, Haltezeit) an **einer** Stelle in der Konfiguration des Audio-Kerns, nicht in `src/scenes`. Reine Funktion (Pegel zum Zeitpunkt t, Ereignisse) mit Test: Pegel sinkt, steigt zurück, **überlappende Warnungen** verlängern das Absenken, statt zu stapeln; Zustandswechsel (Crossfade aus SO4.2) während des Duckings knacken nicht (beide Faktoren multiplizieren sich im Gain, kein Sprung).
- **Musik-Dateien:** nur die in SO4.1 gewählten Stücke (`docs/assets/sounds.md`, Abschnitt Musik) nach `public/audio/musik/<quelle>/`, Format laut SO1.2 (angenommen mp3 mit Fallback, `docs/plan-weiterentwicklung.md` § 11.6, B-166), klein gehalten (Dauer, Bitrate; Größe im Ergebnis nennen: Kaltstart-Budget GR4). Nicht gewählte Kandidaten samt Einträgen in `public/audio/kandidaten.json` entfernen. Musik wird erst bei Bedarf geladen (nicht alle acht beim Start), damit der Ladebalken nicht länger wird; Strategie (z. B. nächster Zustand vorladen) im Ergebnis nennen.
- **Credits (AC-04):** Eintrag je Datei in `public/audio/CREDITS.md` (Format und Anlage aus SO2.2; Kopf `| Ordner | … | Urheber | Lizenz | Quelle |`); CC-BY mit Urheber, Titel, Lizenz und Quelle wörtlich. Die **Credits-Seite** (`lizenzen.html`, Parser `src/tools/credits.ts`, Test `src/tools/credits.test.ts`) kennt `public/audio/` nur, wenn das PLAT-Ticket aus SO2.2 erledigt ist; `src/tools/` gehört PLAT und wird hier **nicht** geändert. Prüfen: Liest `credits.ts` bereits `public/audio/`? Ja → die Test-Aussage „CC-BY-Stücke erscheinen auf der Seite“ ergänzt der dortige Test, hier Beleg im Ergebnis. Nein → Teil „auf der Credits-Seite“ von AC-04 im Ergebnis als `verschoben (Ticket aus SO2.2)` führen, nicht still abhaken; den Teil „Credit-Eintrag je Datei“ prüft ein Test in `src/audio/` (wie `soundKatalog.test.ts` aus SO2.2, auf `public/audio/musik/`).
- **Lizenz:** nur CC0 oder CC-BY (Q15). Musik aus den GothicVania-Packs (Pascal Belisle) ist nicht CC0 und nur mit geprüfter Lizenz und Namensnennung zulässig.
- **Regeln:** `src/scenes` rechnet nichts; eine Musik je Gerät; B-Taste nicht belegen.
- **Hören am TV** (Ducking nach Gehör, Lautheit der Stücke) macht 🧑 in SO4.5; keine Abhängigkeit.

## Erlaubte Dateien

- `src/audio/` (Ducking, Konfiguration, Lade-Strategie, Tests)
- `public/audio/` (Musik-Dateien, `CREDITS.md`, `kandidaten.json`), `docs/assets/sounds.md` (Status der Stücke)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Automat und Crossfade (SO4.2), Änderungen an `src/tools/credits.ts` und `lizenzen.html` (PLAT), Effekt-Sounds (SO2), Optionen-Szene (S5).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. SO4.1-Auswahl, SO4.2- und SO2.3-Ergebnisse lesen.
2. Ducking als reine Funktion mit Test; Anbindung an die Warn-Ereignisse und den Bus „Musik“.
3. Gewählte Stücke nach `public/audio/musik/…` legen, Credits schreiben, nicht gewählte Kandidaten entfernen; Größe der Dateien messen.
4. Credit-Test für `public/audio/musik/`; prüfen, ob die Credits-Seite Audio schon kennt (siehe Kontext).
5. Im Browser-Pane Nacht-Übergang mit `?fast=1` auslösen: Konsole ohne Fehler, Pegel der Musik fällt beim Warn-Ereignis und kehrt zurück (Pegel-Anzeige oder Log des Testaufbaus).
6. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-03: Test des Ducking-Pegels (sinkt bei Warnung, steigt zurück, überlappende Warnungen ohne Stapeln); im Browser-Pane beobachtet.
- [ ] AC-04: Jede Musik-Datei hat einen Credit-Eintrag (Test); CC-BY-Stücke stehen auf der Credits-Seite oder dieser Teil ist mit Ticket als `verschoben` geführt.
- [ ] AC-06: `task check` grün.
- [ ] Nicht gewählte Kandidaten sind entfernt; Gesamtgröße der Musik-Dateien steht im Ergebnis.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
