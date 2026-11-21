# WI3 · SIM · Tiefe Ebenen und Minen: Verteidigung unten, Produktion oben, Erzadern freiräumen

- **Status:** geplant
- **Projekt:** WIR
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** B-1196, B-1197, B-1198
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Heute darf auf jeder Ebene gebaut werden, was die Hub-Stufe freigibt; Erzadern können hinter Steinen und Felsen liegen, ohne dass das Freiräumen vorgeschrieben ist. 🧑-Ideen 2026-11-21: untere Minen nur mit Verteidigung, Produktion nur ganz oben, Ausrüstung von oben holen, Erzadern erst freiräumen.

## Ziel

In den unteren Ebenen stehen nur Verteidigungsanlagen, produzierende Gebäude nur ganz oben; Bürger und Soldaten unten holen Ausrüstung von ganz oben oder vom Boden; Erzadern sind erst nach dem Freiräumen des Wegs zum Hub abbaubar.

## Beteiligte und Zielgruppen

🧑 gibt die Spec frei (Spielregeln, Level-Layout) und nimmt am PC ab.

## Anforderungen

`B-1196`, `B-1197`, `B-1198`. Sessions (bei Aktivierung): WI3.1 Gebäude nach Tiefe (B-1196), WI3.2 Ausrüstung von oben (B-1197), WI3.3 Erzadern freiräumen (B-1198), WI3.4 Review. B-1197 braucht B-1196; B-1198 braucht den Handwerker aus WI2 (B-1194).

## Nicht-Ziele

Werkzeuggürtel und Reparaturkosten (WI2), neue Welten, Balancing.

## Regeln und Einschränkungen

Neue Mechanik nur in Rust (Entscheidung 013); Level-Generator in Rust (`k3c-level`), Go-Golden unverändert; deterministisch, 2 Spieler; Spielstand-Version erhöhen (alte Stände egal); Regeltexte (`materialien-gebaeude.md`, `stufen.md`, `welten.md`, `buerger.md`) ziehen mit; Sim und Balancing gesperrt.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-sim/src/**`, `rust/crates/k3c-level/src/**`, `rust/crates/k3c-data/src/**`, `data/buildings.json`, `data/hub.json`
- **Abhängig von:** WI2
- **Braucht 🧑:** Freigabe | Abnahme
- **Sperre:** golden | spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** (B-1196) Unten nur erlaubte Arten, oben alle; Zuordnung in den Daten.
- **AC-02** (B-1197) Unten ohne Regal holt ein Bürger liegende Ausrüstung oder wandert nach oben.
- **AC-03** (B-1198) Jede Ader hat freiräumbare Hindernisse; Abbau erst nach dem Freiräumen.
- **AC-04** Abnahme 🧑 am PC in einer Mine.

## Offene Fragen

🧑 muss die Spielregeln freigeben. Defaults stehen in B-1196 bis B-1198 (Kampf- und Produktionsgebäude, Reise über Treppen, ein bis drei Hindernisse je Ader).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
