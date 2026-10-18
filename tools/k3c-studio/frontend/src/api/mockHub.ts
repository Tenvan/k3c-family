import fixture from './hubFixture.json';
import type { HubPlanView, HubRulesView, HubSaveResult, HubStand } from './types';

// Gebäude-Planer im Mock (Browser, Tests): Antworten des Backends für die 7-Stufen-Fixture testdata/hub/hub7.json,
// erzeugt vom Go-Backend (app_planer_fixture_test.go hält die Datei gleich). Die Regeltabelle des Checkouts hat 5
// Zeilen und weicht deshalb ab.

export function mockHubPlan(): HubPlanView {
  return structuredClone(fixture.plan) as HubPlanView;
}

/** Abweichung im Mock: immer die der Fixture, unabhängig vom übergebenen Stand (keine Prüfung im Frontend). */
export function mockHubRules(_hubJSON: string): HubRulesView {
  return structuredClone(fixture.rules) as HubRulesView;
}

/** Prüfung im Mock: Matrix und Befunde der Fixture, der Kandidat als Inhalt (das Go-Backend prüft wirklich). */
export function mockHubCheck(candidate: string): HubPlanView {
  return { ...mockHubPlan(), raw: candidate, stand: { hub: '', rules: '' } };
}

/** Speichern im Mock: schreibt nichts, meldet aber wie das Go-Backend (Text von regNotice in app_planer_save.go). */
export function mockHubSave(_candidate: string, stand: HubStand): HubSaveResult {
  const notice = 'REG-Änderung: Commit mit Begründung (data/hub.json und Regeltabelle), task golden:update nur bei gewollter Änderung';
  return { saved: true, unchanged: false, stale: '', findings: [], error: '', rulesError: '', notice, stand };
}
