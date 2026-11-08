import { describe, expect, it } from 'vitest';
import { joinHintDevices, type JoinCandidate } from './joinHint';

const kb = (bound: boolean): JoinCandidate => ({ device: 'keyboard', bound });
const pad = (bound: boolean): JoinCandidate => ({ device: 'pad', bound });

describe('joinHintDevices (B-391)', () => {
  it('vor dem ersten Beitritt: das zuletzt benutzte Gerät, wie bisher', () => {
    expect(joinHintDevices([kb(false), pad(false)], 'keyboard')).toEqual(['keyboard']);
  });

  it('Tastatur beigetreten: Spieler 2 sieht nur den Controller', () => {
    expect(joinHintDevices([kb(true), pad(false)], 'keyboard')).toEqual(['pad']);
  });

  it('jedes Gerät beigetreten: kein Hinweis mehr', () => {
    expect(joinHintDevices([kb(true), pad(true)], 'pad')).toEqual([]);
  });

  it('zwei Controller ergeben nur einen Eintrag „pad“', () => {
    expect(joinHintDevices([kb(true), pad(false), pad(false)], 'keyboard')).toEqual(['pad']);
  });
});
