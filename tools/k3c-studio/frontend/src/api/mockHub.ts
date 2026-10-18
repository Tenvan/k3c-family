import fixture from './hubFixture.json';
import type { HubPlanView, HubRulesView } from './types';

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
