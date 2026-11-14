# HW1.1 · Prüfliste aller offenen Abnahmen am Gerät aus dem Fahrplan

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** hw1/1-pruefliste
- **Abhängig von:** –
- **Tickets:** B-314
- **Kriterien:** AC-01

## Ziel

Eine Prüfliste aller offenen Abnahmen am Gerät (Stand 2026-10-25), sortiert nach Gerät, damit Ralf sie in einem Durchgang (HW1.2) abarbeiten kann.

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

- AC-01 (Prüfliste): umgesetzt. Quelle sind alle Sessions mit `Agent: Mensch` und Status offen auf develop und den offenen Sprint-Branches (git grep, 2026-10-25). Ruhende Projekte (BAL: P1, BR1, BR2, BAL4) sind nicht enthalten.

**TV / Xbox (zwei Controller)**

| Session | Was | braucht PR |
|---|---|---|
| GEB18.7 | Hintergründe: Detail, Abwechslung, Speicher | #465 |
| GEB15.4 | Wurzeln, Felsen, Minen (Dichte 2, Faktor 1,5 bestätigen) | #468 |
| GEB16.7 | Bauschild, dichter Wald, Gerüst, Axt/Spitzhacke | #469 |
| GEB17.8 | Angebot vor der Burg, Wachstumsreihe | #482 |
| W13.4 | Helligkeit Nacht/Morgengrauen, Licht am Lagerfeuer | #480 |
| W11 (ohne Session) | nur erreichbare Bauplätze, Ausbilden/Tauschen im Overlay | #470 |
| ST1 (ohne Session) | Anzeige „gesichert“ alle 60 s | #474 |
| PF1.4 | Split-Screen flüssig, Frame-Zeit im Overlay (`?dev=1`) | #477 |
| W6.4 | Bau, Lager, Hub und Bürger | – |
| HW1.8 | Audio: Entsperren, Format, Split-Screen | – |
| HW1.9 | Hörprobenseite: Bedienung, Crossfade | – |
| HW1.10 | Overlay-Gesten RB und LB + RB | – |
| SO8.4 | Hörprobe der vier Klassen-Töne | – |

**PC**

| Session | Was | braucht PR |
|---|---|---|
| WZ29.9 | Studio: Szenen, Filter, Chat-Vorschlag, Vorrat, Kampf-Anzeigen | #467 |
| KB4.4 | Boss und Event über das Debug-Panel | #476 |
| PM1.4 | `sim_test` mit Client, Messlauf mit `?perf=1` | #472 |
| BD9.4 | Welt im Pausenmenü und in der Lobby, Raum schließen | – |
| WZ28.8 | Werkzeuge am Notebook ohne Internet | – |
| WZ30.7 | alle WZG-Oberflächen im Mock | – |
| WZ31.2 | Workbench und Studio als Web-Client | – |
| HW1.4 | Landingpage und Entwicklerseite mit Tastatur | – |

**Pi**

| Session | Was | braucht PR |
|---|---|---|
| HW1.5 | Messlauf über eine Nacht | – |
| HW1.7 | Release-Checkliste: Pi-Image und Version | – |

**Bestätigungen in KRT** (kein Gerät nötig): B-623 Stufenfolge des Hubs, B-624 Golden als gzip; KR6 Mond-Stichtag (KB1).

Empfohlene Merge-Reihenfolge vor dem Durchgang: zuerst die unabhängigen PRs, dann die Kette #469 → #479 → #480/#481 → #482 → #483.
