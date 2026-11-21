# B-1213 · Mauern: Jägerzaun auf Holz-Level, lückenloser Anschluss und Höhe nach Ausbaustufe

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WL1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Auf dem Holz-Level (Wald) ist die Mauer eine Bretterwand; alle Mauern haben eine kleine Lücke zwischen den Teilen und der Übergang bei Ziegelsteinen passt nicht; die Mauer ist in allen Stufen gleich hoch (Meldung Ralf 2026-11-21).

## Ziel

Holz-Stufe: halbhoher Jägerzaun statt Bretterwand. Alle Mauern schließen lückenlos aneinander, bei Ziegeln passt das Muster am Übergang. Die Höhe wächst mit der Ausbaustufe: Zaun halbe Höhe, Kristall volle heutige Höhe, die Stufen dazwischen aufgeteilt.

## Beteiligte und Zielgruppen

Ralf nimmt am PC ab; Second Worker: Grafik (Assets immer Lane zweit).

## Anforderungen

- **AC-01** Asset Jägerzaun (Holz, Stufe 1) statt der Bretterwand, Papierschnitt-Spec (Entscheidung 005).
- **AC-02** Mauer-Segmente schließen ohne Lücke, Ziegelmuster setzt am Übergang fort.
- **AC-03** Höhe nach Stufe: Faktor 0,5 (Zaun/Stufe 1) bis 1,0 (Kristall/Stufe 5), gleichmäßig aufgeteilt (0,5; 0,625; 0,75; 0,875; 1,0); Darstellung im Spiel, Kollision unverändert.

## Nicht-Ziele

Neue Spielregeln für Mauern (HP, Kosten, Sperre).

## Regeln und Einschränkungen

Entscheidung 005 (Assets nur über MCP `assets_*`); Zaun ist ein halbhoher Holzzaun mit senkrechten, spitz zulaufenden Latten und zwei Querriegeln. Annahme: Stufe n der Mauer entspricht Ausbaustufe n (Holz, Stein, Kupfer, Eisen, Kristall).

Planungsangaben für die Arbeitswellen:

- **Bereich:** `public/grafik/**`, `data/paper.json`, `rust/crates/k3c-game/src/props/**`, `tools/paper/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Wald: Zaun halb so hoch wie die Kristallmauer; zwei Segmente berühren sich ohne Spalt.

## Ausnahme- und Fehlerfälle

Ausbau während sichtbar: Höhe springt auf die neue Stufe (kein Ruckeln nötig).

## Akzeptanzkriterien

- **AC-01** Spec und gerendertes Asset (Review in der Workbench).
- **AC-02** Test: Segmentbreite = Abstand der Plazierung (keine Lücke), Ziegel-Übergang per Bildvergleich.
- **AC-03** Test der Höhenfaktoren je Stufe.

## Offene Fragen

keine.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
