import PAGE from '../../../internal/planning/page.html?raw';
import type { PlanDoc, PlanningData, PlanSession, PlanTicket } from './types';

// Erfundene Planung für den Mock: ein aktiver Sprint mit Session-Tabelle, geplante Entwürfe, Tickets und zwei kurze
// Dokumente, die dieselben Markdown-Formen haben wie docs/plan-weiterentwicklung.md und docs/fragenkatalog.md.

const s = (nr: string, typ: string, agent: string, status: string, titel = '', text?: string): PlanSession =>
  ({ nr, typ, agent, status, titel, text });

const SP11_2 = `# SP11.2 · Pi einrichten

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Tickets:** B-035
- **Kriterien:** AC-02, AC-03

## Ziel

Der Server läuft auf dem Raspberry Pi im Docker und ist im Heimnetz unter Port 8080 erreichbar.

## Erlaubte Dateien

- \`deploy/pi/…\`
- \`docs/betrieb.md\`

## Schritte

1. Image aus SP11.1 auf den Pi ziehen.
2. \`docker compose up -d\` im Ordner \`deploy/pi\`.
3. Von der Xbox \`http://pi.local:8080\` öffnen.

## Fertig, wenn

- [ ] AC-02: \`curl pi.local:8080/api/health\` liefert **ok**.
- [x] AC-03: Neustart des Pi startet den Container mit.
`;
const tk = (nr: string, title: string, domain: string, prio: string, status: string, sprint: string, spec = 'Entwurf'): PlanTicket =>
  ({ nr, title, domain, typ: 'Idee', prio, status, sprint, spec });

const DATA: PlanningData = {
  done: 2,
  sprints: [
    { id: 'SP11', title: 'Raspberry Pi', domain: 'SRV', status: 'aktiv', reife: 'bereit', spec: 'freigegeben', worktree: 'sprint/sp11',
      tickets: ['B-028', 'B-035', 'B-042'],
      sessions: [s('SP11.1', 'Umsetzung', 'autonom', 'fertig', '', `# SP11.1 · Image und Compose

- **Status:** fertig

## Ziel

Ein ARM-Image des Servers liegt in der Registry.
`), s('SP11.2', 'Workshop', 'Mensch', 'offen', '', SP11_2),
        s('SP11.3', 'Workshop', 'Mensch', 'offen'), s('SP11.4', 'Review', 'autonom', 'offen')] },
    { id: 'F1', title: 'Zielkorridore und Bedienungsregeln', domain: 'REG', status: 'geplant', reife: 'Entwurf', spec: 'Entwurf',
      tickets: ['B-134', 'B-135', 'B-136'],
      sessions: [s('F1.1', '', '', 'entwurf', 'Workshop Zielkorridore'), s('F1.2', '', '', 'entwurf', 'Workshop Bedienung 1'),
        s('F1.3', '', '', 'entwurf', 'Workshop Bedienung 2')] },
    { id: 'F2', title: 'Golden-Ablauf, Migration, Determinismus', domain: 'INF', status: 'geplant', reife: 'Entwurf', spec: 'Entwurf',
      tickets: ['B-137', 'B-071'],
      sessions: [s('F2.1', '', '', 'entwurf', 'Golden-Task'), s('F2.2', '', '', 'entwurf', 'Spielstand-Migration')] },
    { id: 'S1', title: 'Monarch-Schlag und Skills', domain: 'SIM', status: 'geplant', reife: 'Entwurf', spec: 'Entwurf',
      tickets: ['B-118', 'B-119'], sessions: [s('S1.1', '', '', 'entwurf', 'Schlag und Pool'), s('S1.2', '', '', 'entwurf', 'Skills')] },
    { id: 'R1', title: 'Regelwerk 1', domain: 'REG', status: 'erledigt', reife: 'bereit', spec: 'freigegeben', tickets: ['B-090'],
      sessions: [s('R1.1', 'Workshop', 'Mensch', 'fertig'), s('R1.2', 'Review', 'autonom', 'fertig')] },
    { id: 'M5', title: 'Dev-MCP-Seite', domain: 'DEV', status: 'erledigt', reife: 'bereit', spec: 'freigegeben', tickets: [],
      sessions: [s('M5.1', 'Umsetzung', 'autonom', 'fertig')] },
  ],
  tickets: [
    tk('B-006', 'Gamepad-Test auf der Xbox ist ausgewertet', 'PLAT', 'hoch', 'eingeplant', 'X1', 'freigegeben'),
    tk('B-007', 'Skill-Baum mit Tank und Zauberer ist spielbar', 'SIM', 'hoch', 'offen', '–'),
    tk('B-011', 'Spiel hat Sound und Musik', 'CLI', 'mittel', 'offen', '–'),
    tk('B-035', 'Server läuft auf dem Raspberry Pi im Docker', 'SRV', 'hoch', 'eingeplant', 'SP11', 'freigegeben'),
    tk('B-112', 'Hub-Ausbau mit Mauerstufen', 'SIM', 'mittel', 'offen', '–'),
    tk('B-134', 'Jede Kennzahl hat einen Zielkorridor als Zahl', 'REG', 'hoch', 'eingeplant', 'F1'),
    tk('B-137', 'Golden-Hashes ändern sich nur mit Begründung', 'INF', 'hoch', 'eingeplant', 'F2'),
  ],
};

const PLAN = `# Plan: K3C nach den Grundlagen

- **Status:** Richtung von 🧑 gewählt; Umsetzung in Tickets B-134 bis B-170 und den Sprints F1 bis RL1.
- **Leitlinie:** früh spielbar, dann Tiefe.

## 1. Ausgangslage

- **Fertig:** Insel/Stufen-Kern (SP12), Optionen (SP13), Raum auf Insel (SP14), Regelwerk R1–R4.
- **Offen:** Hub-Ausbau B-112, Skills B-119, Bosse B-130.

## 3. Phasen und Sprints

### Phase 0 – Fundament

| Sprint | Domäne | Inhalt | Am Ende sichtbar |
|---|---|---|---|
| **F1** | REG | Zielkorridore, Schriftgrößen, Pause-Semantik | Regeln mit Pass/Fail-Zahlen |
| **F2** | INF | Golden-Ablauf, Spielstand-Migration | Rote CI bei Drift |
| **F3** | SIM | Feedback-Events, Benchmark | Events im Debug-Overlay |
`;

const FRAGEN = `# Fragenkatalog

Zu klärende Punkte für die Planung. Eine Frage = ein Absatz mit Optionen und einer **Empfehlung**.

| Nr. | Thema | Blockiert | Block | Status |
|---|---|---|---|---|
| Q01 | Pause im gemeinsamen Raum | F1, S5 | 1 | offen |
| Q02 | Zielkorridore als Zahlen | F1, BAL2 | 1 | offen |
| Q06 | Skill-Tasten am Controller | X1, S3 | 1 | geklärt |

## Block 1 – blockiert F1

### Q01 · Pause im gemeinsamen Raum

- Option A: jeder Spieler kann pausieren, der Raum hält an.
- Option B: nur der Host pausiert.
- **Empfehlung:** A, mit sichtbarem Hinweis, wer pausiert hat.
`;

export function mockPlanning() {
  return {
    planningPage: async (): Promise<string> => PAGE.replace('/*DATA*/null', JSON.stringify(DATA)), // wie planning.Page in Go
    planningDoc: async (name: PlanDoc): Promise<string> => (name === 'plan' ? PLAN : FRAGEN),
  };
}
