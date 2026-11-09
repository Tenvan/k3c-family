# BD11 · CLI · Eine Startseite, Navigation im Pausenmenü, Vollbild halten

- **Status:** geplant
- **Projekt:** BED
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-768
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Landingpage mit Kacheln und Iframe-Seiten, Lobby in `game.html`, Pausenmenü im Spiel; Vollbild geht beim Wechsel leicht verloren (Xbox, Edge). Details: B-768.

## Ziel

Ein Einstieg: eine Startseite mit Bereichswechsel per links/rechts, Pad und Schaltflächen; Neues Spiel und Fortsetzen nur im Pausenmenü; andere Seiten nur über das Pausenmenü; der Client hält Vollbild. Am Ende sichtbar: Xbox: Start → Spiel → Pausenmenü → Seiten → zurück, alles per Pad, Vollbild bleibt.

## Beteiligte und Zielgruppen

Spieler am TV (Xbox-Pad), Entwickler am PC; 🧑 gibt die Spec frei und nimmt am Gerät ab.

## Anforderungen

Siehe B-768 › Anforderungen (Startseite, Pausenmenü als einziger Weg, Vollbild halten, Pad-Bedienung).

## Nicht-Ziele

Kein neues Menü-Design, keine Änderung an Protokoll oder Spielständen, Werkzeugseiten bleiben inhaltlich unverändert.

## Regeln und Einschränkungen

Shell-Regeln aus CLAUDE.md (Landingpage bleibt offen, `openPage()`, `toggleFullscreen()`, `installPageChrome()`, B frei, View+Menu reserviert); Texte aus `src/core/texts.*.ts`.

## Beispiele

Siehe B-768 › Beispiele.

## Ausnahme- und Fehlerfälle

Siehe B-768 › Ausnahme- und Fehlerfälle (Vollbild abgelehnt, kein Server, Spiel im Gange).

## Akzeptanzkriterien

- **AC-01** Startseite wechselt Bereiche per links/rechts, Pad und Schaltflächen (`B-768/AC-01`, Test).
- **AC-02** Neues Spiel und Fortsetzen nur im Pausenmenü (`B-768/AC-02`, Test).
- **AC-03** Alle Seiten aus dem Pausenmenü erreichbar, keine Querlinks (`B-768/AC-03`, projectRules).
- **AC-04** Vollbild bei der ersten Eingabe und nach Verlust erneut (`B-768/AC-04`, Test).
- **AC-05** Abnahme am Gerät: Xbox/Edge, alles per Pad, Vollbild bleibt (`B-768/AC-05`).

## Offene Fragen

- 🧑: Lobby (Räume, Beitritt) als Einträge im Pausenmenü (Vorschlag: ein Eintrag je Aktion)?
- 🧑: Werkzeugseiten im Pausenmenü unter „Seiten“ (Vorschlag)?

## Sessions

Stichpunkte (Reife Entwurf):

- BD11.1 CLI Umsetzung: Startseite mit Bereichswechsel und Vollbild-Halten (AC-01, AC-04).
- BD11.2 CLI Umsetzung: Pausenmenü mit Neues Spiel, Fortsetzen und Seitenwechsel; Lobby und Querlinks entfallen (AC-02, AC-03).
- BD11.3 CLI Review: Sprint BD11 abnehmen (alle).
- BD11.4 CLI Workshop Mensch live: Abnahme an der Xbox (AC-05).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
