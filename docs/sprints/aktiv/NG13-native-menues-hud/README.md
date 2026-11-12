# NG13 · CLI · Natives Spiel: Menüs und HUD nachziehen

- **Status:** aktiv
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-807, B-808, B-809, B-812, B-813, B-811
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Der Funktionsabgleich NG9.1 zeigt Lücken in Menüs und HUD des nativen Spiels: keine Tastenbelegung im Pause-Menü, keine Lizenzen und Version, keine Schalter für Blitze und Anleitung, schlichtes Text-Menü ohne Papier-Optik, Fehler bei Burgfall-Banner, Skill-Menü, Hauptmenü nach dem Verlassen und Siegmeldung, Anleitung ohne Bilder für Münze und Mond. Einzelheiten: B-807, B-808, B-809, B-812, B-813, B-811 (AC-03).

## Ziel

Menüs und HUD des nativen Spiels haben den Funktionsumfang des Browser-Clients (ohne Kachel-Illustrationen) und zeigen die Befunde aus NG9.1 nicht mehr. Am Ende sichtbar: Tests in `menu`, `hud`, `fx`, `local` grün, `task native:game:e2e` mit zwei gleichen Läufen, Lizenzdatei in `.work/native-game/`.

## Beteiligte und Zielgruppen

Spieler am PC und Mini-PC; Claude (Umsetzung, Review); 🧑 Ralf (Sichtprüfung bei der nächsten PC-Abnahme, nicht blockierend).

## Anforderungen

`B-807 › Anforderungen`, `B-808 › Anforderungen`, `B-809 › Anforderungen`, `B-812 › Anforderungen` (Revision 2, ohne Illustrationen), `B-813 › Anforderungen`, `B-811 › Anforderungen` nur Anleitung mit Münze und Mond.

## Nicht-Ziele

Kachel-Illustrationen (B-818), Figuren-, Baum- und Geschoss-Anzeigen (NG12), Shader-Umschalter (NG14), Umbelegen von Tasten, Sim- und Balance-Läufe.

## Regeln und Einschränkungen

Entscheidung 008; Importregel aus `native/k3c-game/view/doc.go` (geprüft in `game/imports_test.go`): `menu` bekommt die Tastenbelegung über den Vertrag `view` (B-807 › Offene Fragen); `view` und `view.Settings` ändern sich nur additiv. Textmaße gelten mit der Schrift aus NG18. Der Ende-zu-Ende-Lauf (`TestFlow`) ist Regressionsschutz. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/menu/**`, `native/k3c-game/hud/**`, `native/k3c-game/fx/**`, `native/k3c-game/view/**`, `native/k3c-game/game/**`, `native/k3c-game/local/settings.go`, `native/k3c-game/local/settings_test.go`, `native/Taskfile.yml`
- **Abhängig von:** NG12, NG14, NG18
- **Braucht 🧑:** –
- **Sperre:** taskfile

## Beispiele

`B-807 › Beispiele`, `B-808 › Beispiele`, `B-809 › Beispiele`, `B-812 › Beispiele`, `B-813 › Beispiele`.

## Ausnahme- und Fehlerfälle

Je Ticket unter „Ausnahme- und Fehlerfälle“. Einstellungsdatei fehlt oder ist kaputt → Standardwerte, Log-Zeile (B-809). Version nicht ermittelbar → „unbekannt“ (B-808).

## Akzeptanzkriterien

- **AC-01** Seite Tasten im Pause-Menü, eine Zeile je Aktion, Zurück ins Pause-Menü (`B-807/AC-01`).
- **AC-02** Importregel-Test grün (`B-807/AC-02`).
- **AC-03** Ende-zu-Ende-Lauf gleich in allen Stationen (`B-807/AC-03`).
- **AC-04** Lizenz-Seite nennt Ebitengine, Schrift und Audio (`B-808/AC-01`).
- **AC-05** Hauptmenü zeigt Version, ohne Angabe „unbekannt“ (`B-808/AC-02`).
- **AC-06** Lizenzdatei im Paket (`B-808/AC-03`).
- **AC-07** Schalter Blitze und Anleitung bleiben nach Speichern und Laden (`B-809/AC-01`).
- **AC-08** Anleitung aus → kein Hinweis (`B-809/AC-02`).
- **AC-09** „Anleitung gesehen“ überlebt einen Neustart (`B-809/AC-03`).
- **AC-10** Karten im Bild ohne Überlappung, Auswahl hervorgehoben (`B-812/AC-01`).
- **AC-11** Farben aus `stil.paper.json` (`B-812/AC-02`).
- **AC-12** Ende-zu-Ende-Lauf mit Papier-Menü, Bilder abgelegt (`B-812/AC-03`).
- **AC-13** Burgfall-Banner im Bild, Verluste des Modus `lost` (`B-813/AC-01`).
- **AC-14** Skill-Menü ohne Überlappung (`B-813/AC-02`).
- **AC-15** Erstes Draw nach „Spiel verlassen“ ohne falschen Zustand (`B-813/AC-03`).
- **AC-16** `victory` unabhängig von der Stufe des lokalen Spielers (`B-813/AC-04`).
- **AC-17** Anleitung mit Münze und Mond als Bild (`B-811/AC-03`).

## Offene Fragen

keine (entschieden autonom, Claude, 2026-11-12: B-807 über `view`, B-809 Schalter kommen dazu, B-812 zerlegt; siehe die Tickets).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG13.1 | `NG13.1-tastenbelegung.md` | Umsetzung | autonom | offen |
| NG13.2 | `NG13.2-lizenzen-version.md` | Umsetzung | autonom | offen |
| NG13.3 | `NG13.3-optionen-blitze-anleitung.md` | Umsetzung | autonom | offen |
| NG13.4 | `NG13.4-papier-karten.md` | Umsetzung | autonom | offen |
| NG13.5 | `NG13.5-hud-befunde-anleitung.md` | Umsetzung | autonom | offen |
| NG13.6 | `NG13.6-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
