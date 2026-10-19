import { describe, expect, it } from 'vitest';
import { joinHintDevices, type JoinCandidate } from './joinHint';

const kb = (bound: boolean): JoinCandidate => ({ device: 'keyboard', bound });
const kb2 = (bound: boolean): JoinCandidate => ({ device: 'keyboard2', bound });
const pad = (bound: boolean): JoinCandidate => ({ device: 'pad', bound });

describe('joinHintDevices (B-391)', () => {
  it('vor dem ersten Beitritt: das zuletzt benutzte Gerät, wie bisher', () => {
    expect(joinHintDevices([kb(false), kb2(false)], 'keyboard')).toEqual(['keyboard']);
  });

  it('Spieler 1 an der Tastatur beigetreten: Spieler 2 sieht Enter', () => {
    expect(joinHintDevices([kb(true), kb2(false)], 'keyboard')).toEqual(['keyboard2']);
  });

  it('mit Controller daneben: beide freien Geräte, das zuletzt benutzte zuerst', () => {
    expect(joinHintDevices([kb(true), kb2(false), pad(false)], 'pad')).toEqual(['pad', 'keyboard2']);
  });

  it('jedes Gerät beigetreten: kein Hinweis mehr', () => {
    expect(joinHintDevices([kb(true), kb2(true)], 'keyboard2')).toEqual([]);
  });

  it('zwei Controller ergeben nur einen Eintrag „pad“', () => {
    expect(joinHintDevices([kb(true), kb2(true), pad(false), pad(false)], 'keyboard')).toEqual(['pad']);
  });
});
