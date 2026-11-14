# B-886 · Abnahme MG10: Sichtprüfung der Bosse und Gen-Läufe sind erledigt

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint MG10 (B-720, B-711) ist umgesetzt und reviewt; offen sind die Sichtprüfung der Boss-Silhouetten durch 🧑 (B-720/AC-01) und die Gen-Läufe am Sprint-Ende (`docs/arbeitsweise.md` › Sim, Balance und Gen).

## Ziel

MG10 ist abgenommen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- 🧑 sichtet im Review der Workbench die Gruppe `bosse` (troll-koenig, lava-golem, splitter-titan, ratten-koenigin, goblin-anfuehrer; je idle, lauf, angriff), Silhouetten nebeneinander unterscheidbar; danach B-720 erledigt.
- 🧑 bestätigt B-711: Der Client nutzt die flachen Reit-Sheets, die Satteldecke trägt dort die Spielerfarbe (die Ebenen ohne Spielerfarbe nutzt der Client nicht).
- `task gen:paper` samt Grafik-Golden und `task check:race` einmal grün.

## Nicht-Ziele

Neue Grafik; Befunde der Sichtprüfung werden eigene Tickets. Nativer Rig-Export: B-885.

## Regeln und Einschränkungen

Gen nur im Abnahme-Ticket (B-742).

- **Bereich:** `public/grafik/**`
- **Abhängig von:** B-720
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** golden

## Beispiele

nicht relevant: Abnahme ohne neue Logik.

## Ausnahme- und Fehlerfälle

Golden-Abweichung bei `gen:paper` → Ursache klären, kein blindes Neuschreiben.

## Akzeptanzkriterien

- **AC-01** Sichtprüfung durch 🧑 dokumentiert (je Boss ✅ oder Ticket).
- **AC-02** `task gen:paper` ohne Golden-Abweichung, `task check:race` grün (ohne gcc: als übersprungen nennen).

## Offene Fragen

keine

## Notizen

Quelle: MG10.4.
