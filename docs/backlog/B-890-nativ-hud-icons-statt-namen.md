# B-890 · Das HUD des nativen Spiels zeigt Ressourcen als Icon mit Zahl statt als Namen

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU61
- **Projekt:** RSU
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Das HUD des nativen Spiels nennt Ressourcen mit Namen (Münzen, Holz, Stein …). Das liest sich wie eine Tabelle, nicht wie ein Spiel, und ist auf dem TV im Split-Screen schwer zu erfassen. MG11 hat die Papierschnitt-Icons neu gemacht (`public/grafik/icons/`, Zuordnung `data/paper.json`).

## Ziel

Das HUD zeigt je Ressource Icon und Zahl, gut lesbar auf dem TV (Xbox-Abstand) und im Split-Screen für 2 Spieler.

## Beteiligte und Zielgruppen

- 🧑 Ralf: Abnahme auf TV und PC.
- Spielende: lesen den Vorrat auf einen Blick.

## Anforderungen

- Jede Ressource des HUD (Münzen, Holz, Stein, Kupfer, Eisen, Kristall … je nach vorhandenen Ressourcen) erscheint als Icon + Zahl statt Name + Zahl.
- Icons nur aus den vorhandenen Papierschnitt-Assets; keine neuen Assets zeichnen. Fehlt ein Icon: Ticket anlegen, vorerst ein vorhandenes Icon oder ein Kürzel.
- Gut lesbar auf TV und im Split-Screen für 2 Spieler.

## Nicht-Ziele

- Neue Icons zeichnen (eigenes Ticket, falls eins fehlt).

## Regeln und Einschränkungen

Grafik nur über die Papierschnitt-Assets (Entscheidung 005). Datei ≤ 400 Zeilen, Funktion ≤ 60.

Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game`; die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-game/src/hud/**`, `data/paper.json`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

- Spieler 1 hat 12 Münzen und 5 Holz → HUD zeigt Münz-Icon 12, Holz-Icon 5.

## Ausnahme- und Fehlerfälle

- Ressource ohne Icon → Kürzel statt Icon, der Test meldet die Lücke nicht als Fehler, wenn sie als Ticket vermerkt ist.

## Akzeptanzkriterien

- **AC-01** Jede Ressource der Sim hat ein Icon im HUD (Go-Test der Zuordnung Ressource → Icon).
- **AC-02** Das HUD zeichnet Icon + Zahl statt Name + Zahl (Aufnahme als Nachweis).
- **AC-03** Sichtprüfung auf TV und im Split-Screen durch 🧑.

## Offene Fragen

Keine (von 🧑 am 2026-11-14 freigegeben).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
