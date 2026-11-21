# WL1.4 · Jägerzaun, lückenlose Mauern und Höhe nach Stufe

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wl1/4-zaun-mauern
- **Abhängig von:** –
- **Tickets:** B-1213
- **Kriterien:** AC-04

## Ziel

Ein bis zwei Sätze: welches Ergebnis nach dieser Session auf `develop` liegt.

## Kontext

Alles, was ein Agent **ohne Vorwissen** braucht: Stand des Codes, relevante Dateien mit Pfad, Entscheidungen
(`docs/decisions/…`), Fallstricke. Keine Verweise auf Chat-Verläufe.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.
Ein Pfad oder Glob je Zeile in Backticks: daraus entsteht die Kollisionsprüfung paralleler Sessions. Sammeldateien (`docs/backlog/README.md`,
`docs/sprints/README.md`, `docs/projekte/README.md`, `docs/glossar.md`) stehen hier nicht; sie ändert `plan_*` oder der letzte Commit des Sprints.

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
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-04 geprüft (B-1213 AC-01 bis AC-03), mit Abweichungen: Die Session-Datei war eine leere Vorlage; umgesetzt nach dem Ticket B-1213.
- AC-02 Lücke: Die Mauer-Grafik hatte je Seite ca. 6,5 px leeren Rand im Rahmen (Kunst 29 von 42 px). `mauer.paper.json` Rahmen jetzt `[-17,-191,34,191]` (per `assets_set`, Golden `gebaeude-paper/golden.json` neu), die Hintergrundmauer kachelte schon ohne Fugen (`kachelung` 6..39). Tests `walls_tests.rs`: Rahmen ohne leeren Rand.
- AC-03 Höhe: `props/buildings.rs::wall_height` (Stufe 1 bis 5: 0,5 · 0,625 · 0,75 · 0,875 · 1,0), gesetzt als `Item.sy` an der gebauten Mauer (`plan_sites.rs`) und an der Hintergrundmauer (`paint.rs::tiles` liest `sy`); Kollision unverändert. Tests `walls_tests.rs`, `plan_sites_tests.rs::mauer_zeigt_die_hoehe_der_stufe`.
- AC-01 Jaegerzaun: Stufe 1 war schon ein Holzzaun aus spitzen Pfählen mit zwei Querriegeln (B-1176); er wird jetzt halb so hoch gezeichnet (Faktor 0,5). Kein neues Asset nötig; Ralf entscheidet bei der Abnahme, ob die Optik (gestauchter Zaun) reicht oder ein eigenes Zaun-Asset nötig ist. Ziegel-Übergang: kein Bildvergleich gefahren, bei der Abnahme am PC prüfen.
- Zusatz: `tests/still_hud.rs` auf 400 Zeilen gebracht und fmt-sauber (stand unformatiert auf main, WL1.3; `changed` deckt die Beitreten-Meldung 12 ab).
- `cargo test -p k3c-game`: nur `alloc_je_bild` rot (941 > 786, bekannt, B-1190 AC-04). fmt, clippy, Regeltests grün.

### Für den Hauptrechner

- Optik am PC abnehmen (Zaun halbhoch, Mauern ohne Spalt, Ziegel-Übergang, Hintergrundmauer folgt der Höhe); bei Bedarf Ticket für ein eigenes Zaun-Asset mit kleinerer Kunst statt Stauchung.
