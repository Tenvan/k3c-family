# B-808 · Das native Spiel zeigt Lizenzen und Version und liefert die Lizenzdatei im Paket mit

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Browser-Client hat eine Lizenz-Seite (`src/scenes/menuPages.ts:74-83`, `lizenzen.html`) und zeigt die Version. Das native Spiel zeigt weder noch, und das Paket für den Mini-PC (`task native:game:paket`) enthält keine Lizenzdatei. Ebitengine, Schrift und Audio-Bibliotheken verlangen Lizenzhinweise.

## Ziel

Das native Spiel nennt seine Lizenzen und die Version, und das Paket enthält die Lizenzdatei.

## Beteiligte und Zielgruppen

Spieler, Ralf (Weitergabe des Pakets), Claude (Umsetzung).

## Anforderungen

- Lizenz-Seite im Menü (Ebitengine, Schrift, Audio) wie `menuPages.ts:74-83`.
- Versionsanzeige im Hauptmenü.
- Lizenzdatei im Paket unter `.work/native-game/`.
- Texte aus einer Quelle, nicht dreifach gepflegt.

## Nicht-Ziele

Neue Abhängigkeiten, Lizenz der Schrift (kommt mit B-815).

## Regeln und Einschränkungen

Glossar beachten, Datei ≤ 400 Zeilen. Der Task `native:game:paket` löscht nur unter `.work/`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/menu/**`, `native/Taskfile.yml`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

Hauptmenü zeigt „Version 0.x“; Info → Lizenzen listet Ebitengine mit Lizenztext.

## Ausnahme- und Fehlerfälle

Version nicht ermittelbar (Build ohne Angabe) → „unbekannt“, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Test: Lizenz-Seite nennt Ebitengine, Schrift und Audio.
- **AC-02** Test: Hauptmenü zeigt eine Version, ohne Angabe „unbekannt“.
- **AC-03** `task native:game:paket` legt die Lizenzdatei ins Paket; geprüft an der Paket-Dateiliste.

## Offene Fragen

keine

## Notizen

Gefunden im Funktionsabgleich NG9.1 (2026-11-11).
