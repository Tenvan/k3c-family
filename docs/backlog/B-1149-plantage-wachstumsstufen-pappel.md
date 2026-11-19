# B-1149 · Plantage-Wachstumsstufen wieder unterscheidbar machen

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** LA1
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Plantage nutzt `pappel-niedrig` (`props/nature.rs`, `plant_tree_look`). Nach BB1.3 ist sie 2,6 M hoch (vorher rund 1,8 M); die Wachstumsstufen Setzling, Jungbaum und Altbaum sind kaum noch unterscheidbar (vermutet).

## Ziel

Drei Wachstumsstufen der Plantage sind am TV auf einen Blick unterscheidbar.

## Beteiligte und Zielgruppen

Spieler am TV; Umsetzung Claude (CLI); Abnahme: Ralf.

## Anforderungen

- Stufenhöhen spürbar gestaffelt (z. B. Setzling 0,8 M, Jungbaum 1,6 M, Altbaum bis 3 M) oder die neuen Varianten pappel-niedrig/pappel/pappel-hoch je Stufe.
- Keine Änderung der Spielregeln (Wachstumszeit bleibt).

## Nicht-Ziele

Neue Assets (nur wenn die vorhandenen Varianten nicht reichen: eigenes Ticket).

## Regeln und Einschränkungen

Das Spiel zeichnet nur. Planungsangaben:
- **Bereich:** `rust/crates/k3c-game/src/props/nature.rs`, `data/paper.json`
- **Abhängig von:** B-1146
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Frisch gepflanzt: kniehoher Setzling; nach der Reife: ausgewachsene Pappel.

## Ausnahme- und Fehlerfälle

Unbekannte Stufe: Altbaum zeichnen.

## Akzeptanzkriterien

- **AC-01** Rust-Test: die drei Stufen liefern drei verschiedene Anzeigehöhen, Abstand mindestens 0,5 M.
- **AC-02** Abnahme am TV (Ralf): Stufen unterscheidbar.

## Offene Fragen

keine

## Notizen

Von BB1.5 angelegt (Befund BB1.3).
