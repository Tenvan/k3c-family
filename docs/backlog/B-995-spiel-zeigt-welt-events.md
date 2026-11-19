# B-995 · Das Spiel kündigt Überfall, Sandsturm und Nebel an und zeigt sie in der Nacht

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WE8
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Das native Spiel (`native/k3c-game`, `hud/`, `fx/`, `view/`) zeigt Vollmond und Blutmond. Die neuen Events Überfall (B-991), Sandsturm und Nebel (B-992) haben keine Anzeige.

## Ziel

Spieler erkennen vor und in der Nacht, welches Event gilt, und sehen Sandsturm und Nebel im Bild.

## Beteiligte und Zielgruppen

Spieler (2 Spieler, Split-Screen). 🧑 gibt frei und prüft am PC.

## Anforderungen

- Ankündigung je Event in der Abenddämmerung an der Stelle, an der Vollmond und Blutmond angekündigt werden, mit Name aus den Daten.
- Sandsturm: Schleier und treibender Sand über der Oberfläche; Nebel: heller Schleier mit begrenzter Sichttiefe. Nur in Tiefe 0, nur in der Event-Nacht.
- Überfall: Hinweis beim Eintreffen der Zusatzgruppe.
- Jede Bildhälfte zeigt nur, was für die Stufe ihres Spielers gilt.
- Kein Einbruch der Bildrate (Schleier als eine Fläche, keine Partikelmassen); das Spiel zeichnet nur.
- Log-Meldungen mit Emoji nach Regel (🌙).

## Nicht-Ziele

Ton für die Events (eigenes Ticket bei Bedarf), neue Papierschnitt-Assets, Spiellogik.

## Regeln und Einschränkungen

Das Rust-Spiel zeichnet nur. Controller-Belegung unverändert.

Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game`; die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-game/src/**`
- **Abhängig von:** B-991, B-992, B-994, RU37
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Nil, 6. Abend → Ankündigung „Sandsturm“, in der Nacht Schleier über der Oberfläche; Spieler 2 in der Höhle sieht keinen Schleier.

## Ausnahme- und Fehlerfälle

Zwei Events in derselben Nacht → beide Ankündigungen nacheinander, Schleier überlagern sich nicht doppelt (der stärkere gilt). Unbekanntes Event im Zustand → Name aus den Daten, kein Schleier, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Test: Aus dem Zustand mit aktivem Event entstehen Ankündigung und Schleier nur für Tiefe 0.
- **AC-02** Test: unbekanntes Event und zwei Events gleichzeitig.
- **AC-03** Sichtprüfung 🧑 am PC (Teil der Abnahme von B-996).

## Offene Fragen

- Darstellung → Default: halbtransparente Fläche mit langsam laufendem Muster, keine Partikel (vorläufig, 🧑 bestätigt).

## Notizen

Zerlegt aus B-960 (Events, Anteil Spiel).
