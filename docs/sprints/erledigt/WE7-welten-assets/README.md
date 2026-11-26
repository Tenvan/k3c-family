# WE7 · PAP · Papierschnitt-Assets für Sengoku, Nil und Britannien

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-988, B-989, B-990
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Sengoku, Nil und Britannien (WE3) nutzen nur vorhandene Papierschnitt-Assets (B-960). Assets entstehen nur über `assets_*` (Entscheidung 005).

## Ziel

Die drei Welten haben eigene Bilder. Am Ende sichtbar: 14 neue Assets im Bereich Assets der Workbench (Review), je Welt gebündelt.

## Beteiligte und Zielgruppen

Claude legt an, Agent `k3c-grafik` prüft, 🧑 nimmt im Bereich Assets ab.

## Anforderungen

Siehe `B-988 › Anforderungen`, `B-989 › Anforderungen`, `B-990 › Anforderungen`. Sprint-eigen: Jedes Skin-Asset ist deckungsgleich mit seinem Basis-Asset (Zustände, Animationen, Anker), damit B-994 es ohne Code tauscht.

## Nicht-Ziele

Zuordnung in den Welten (B-996, WE8), Zeichnen der Skins (WE6), `gen:paper` und Grafik-Golden in den Umsetzungs-Sessions (nur Abnahme-Ticket, B-742).

## Regeln und Einschränkungen

Der Sprint hängt von nichts ab und läuft parallel zu WE5 und WE6. Die drei Asset-Sessions laufen nacheinander (gemeinsame `index.paper.json` je Gruppe). Die Workbench muss laufen (`assets_*`); im Worktree immer `checkout` setzen.

- **Bereich:** `public/grafik/objekte/**`, `public/grafik/gegner/**`, `public/grafik/gebaeude-paper/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

`assets_list` zeigt `gegner/ninja`; `assets_get` nennt dieselben Animationen wie `gegner/wolf`.

## Ausnahme- und Fehlerfälle

Ein Basis-Asset liegt in einer anderen Gruppe als angenommen (Schiff, Turm, Bauernhaus) → das Ersatz-Asset kommt in dieselbe Gruppe wie die Basis; liegt die Gruppe außerhalb des Bereichs, Ticket anlegen und im Ergebnis melden.

## Akzeptanzkriterien

- **AC-01** Assets Sengoku angelegt und deckungsgleich mit der Basis (B-988/AC-01, AC-02).
- **AC-02** Assets Nil (B-989/AC-01, AC-02).
- **AC-03** Assets Britannien (B-990/AC-01, AC-02).
- **AC-04** Agent `k3c-grafik` ohne schwere Befunde; Abnahme-Ticket für Gen und Grafik-Golden angelegt; Sichtprüfung 🧑 offen in der Ralf-Queue.

## Offene Fragen

Keine blockierende. Default Basis je Asset steht in B-988, B-989, B-990 (vorläufig, 🧑 bestätigt).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WE7.1 | `WE7.1-assets-sengoku.md` | Umsetzung | autonom | fertig |
| WE7.2 | `WE7.2-assets-nil.md` | Umsetzung | autonom | fertig |
| WE7.3 | `WE7.3-assets-britannien.md` | Umsetzung | autonom | fertig |
| WE7.4 | `WE7.4-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-26, mit WZG geschlossen (🧑 „Rest verschieben“): offene Tickets verschoben nach WN2, keine Abnahme erteilt; Abnahme-Ticket B-1286.
