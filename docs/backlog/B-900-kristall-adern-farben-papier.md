# B-900 · Kristall-Adern gibt es als Papierschnitt in drei Farben (blau, violett, grün)

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BI2
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: nächste autonome Welle)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Kristallhöhle hat seit B-891 die Gewichts-Gruppe `crystalColors` (blau, violett, gruen): Jede Kristall-Ader (`crystalVein`) trägt eine Farbe als Variante. Es gibt nur ein Asset `ader-kristall`.

## Ziel

Drei Papierschnitt-Assets der Kristall-Ader, je Farbe eins, über `assets_*` angelegt und im Studio geprüft.

## Beteiligte und Zielgruppen

Spieler der Kristallhöhle; Claude legt die Assets an; Ralf 🧑 prüft sie im Abnahme-Ticket.

## Anforderungen

- Blau ist das vorhandene `ader-kristall` (Material `kristall`).
- Neu: `ader-kristall-violett` (Material `purpur`) und `ader-kristall-gruen` (Material `smaragd`) in der Gruppe `objekte`, Form, Lagen, Ausgabe und Rahmen wie `ader-kristall`, nur die Kristall-Lagen tauschen das Material.
- Eintrag in `data/paper.json` wie `ader-kristall`.

## Nicht-Ziele

Änderungen an der Stil-Datei; Leucht-Effekte (B-906); Einbau ins Spiel (B-901).

## Regeln und Einschränkungen

Assets nur über `assets_*` (Entscheidung 005), Stil laut `assets_guide`. PNG und Manifest sind nicht eingecheckt (B-737); gerendert wird im Abnahme-Ticket.

- **Bereich:** `public/grafik/objekte/**`, `data/paper.json`
- **Abhängig von:** B-891
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

`assets_list gruppe objekte` → `ader-kristall`, `ader-kristall-violett`, `ader-kristall-gruen` gelten als gültig.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Grafik ohne Laufzeit-Verhalten.

## Akzeptanzkriterien

- **AC-01** Je Farbe (blau, violett, gruen) gibt es eine gültige Spec der Kristall-Ader mit derselben Ausgabe-Größe und demselben Rahmen wie `ader-kristall`; violett nutzt `purpur`, grün `smaragd` statt `kristall`.
- **AC-02** `data/paper.json` nennt beide neuen Assets mit denselben Maßen wie `ader-kristall`.

## Offene Fragen

keine (blau = vorhandenes Asset; Materialien `purpur`/`smaragd` statt neuer Stil-Töne, da die Stil-Datei nur Sessions mit Lane stil ändern).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
