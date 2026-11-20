# B-1156 · Kritische Entscheidungen aus RU59 bis RU62 (selbst entschieden, zur Durchsicht)

- **Domäne:** INF
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** KR6
- **Projekt:** KRT
- **Erstellt:** 2026-11-20
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ralf (2026-11-20): Kritische Fragen selbst entscheiden und hier sammeln. Entschieden wurde in den Sprints RU59 bis RU62 (Claude, autonom).

## Ziel

Ralf sieht an einer Stelle, was ohne Rückfrage entschieden wurde, und ändert, was ihm nicht gefällt.

## Beteiligte und Zielgruppen

Ralf (🧑) entscheidet, ob er eine Entscheidung zurückdreht.

## Anforderungen

- Entscheidungen unten prüfen; Widerspruch als eigenes Ticket.

## Nicht-Ziele

Neue Mechaniken.

## Regeln und Einschränkungen

- **Bereich:** `docs/backlog/**`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: Sammelticket.

## Ausnahme- und Fehlerfälle

nicht relevant: Sammelticket.

## Akzeptanzkriterien

- **AC-01** Ralf hat die Entscheidungen unter Notizen gesehen und, wo nötig, Gegenentscheidungen als Ticket angelegt.

## Offene Fragen

Entschieden (Claude, 2026-11-20, mit Default):

1. **Bildvergleiche (RU61.5):** Schwellen von `still`, `still_hud`, `still_props` an den neuen Look angepasst (nach Ralfs „Ja, entscheide“); fünf Props-Stände haben keine Pixel-Referenz mehr, nur eine grobe Obergrenze. Alternative: neue Rust-Referenzbilder erzeugen.
2. **Wetter-Ton (RU62.7):** `sfx:wetter-wind` und `sfx:wetter-wasser` zeigen auf Platzhalter (`kenney-rpg-audio-drawknife1`, `kenney-impact-sounds-impactsoft-medium-0`), weil im Repo keine Wind- oder Wasserquelle liegt. Alternative: echte Dateien besorgen (B-936).
3. **Festmusik (RU62.13):** vorhandene Stücke wiederverwendet (Sieg-, Abend- und Tagesmusik als `music:fest*`), keine neue Audiodatei. Klangbild-Ausnahme bis B-936.
4. **Welt-Tier am Gatter (RU62.9):** Platzhalter aus Rechtecken, weil das Reittier ein Rig ohne Standbild ist.
5. **Einsiedler-Figur (RU62.11):** nach der Zahlung sofort leere Hütte, kein sichtbarer Weg zum Abgang.
6. **Tanz im Fest (RU62.13):** Schlag-Pose in Schleife, gespiegelt und versetzt; arbeitende und tanzende Bäuer sehen gleich aus.
7. **RU59.1 fertig trotz AC-03-Lücke:** `golangci-lint` fehlt im PATH; `check:go` formal rot, `check:dev`, `check:race`, Dev-Instanz nicht gefahren (nur Konstante und Go-Test geändert). `deadcode` nicht ausgeführt (Download abgelehnt), 71 Namen unberührt.
8. **`detvec` (RU59.1):** Go-Test nimmt `paper.json`-Zahlen aus den Daten und kennt sechs Rust-only-Dateien; Golden unverändert.
9. **Reviews ohne Agenten (RU62.14, RU59.3, RU61.7):** Selbst-Review am Diff, kein `k3c-reviewer`/`k3c-grafik`/`k3c-sound`.
10. **PR-Merge:** PRs gemäß Ralfs Auftrag (2026-11-20) in `main` gemergt.

## Notizen

Quelle: Sessions RU59.1, RU61.5, RU62.7, RU62.9, RU62.11, RU62.13.
