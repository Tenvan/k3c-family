import { useState } from 'react';
import { Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { Tabs } from '@webkit/organisms';
import { loadPref, savePref } from '../lib/prefs';
import { ConfigTab } from './ConfigTab';
import { DetailHead, type DetailActions } from './DetailHead';
import type { ListEntry } from './entries';
import { FiguresTab } from './FiguresTab';
import { HintsTab } from './HintsTab';
import { OverviewTab } from './OverviewTab';
import { TargetsTab } from './TargetsTab';
import { useInsights, useSummary } from './useBalancing';

export const TABS = ['ueberblick', 'ziele', 'diagramme', 'hinweise', 'konfig'] as const;
export type DetailTab = (typeof TABS)[number];

interface Props {
  entry: Extract<ListEntry, { type: 'report' }>;
  actions: DetailActions;
}

/** Bericht (B-660): Kopf mit Aktionen und fünf Reiter, der gewählte Reiter bleibt gemerkt. */
export function ReportDetail({ entry, actions }: Props) {
  const info = entry.report;
  const { summary, error } = useSummary(info.path);
  const { insights, error: insightsError } = useInsights(info.path);
  const [tab, setTab] = useState<DetailTab>(() => loadPref('balancing.tab', TABS, 'ueberblick'));
  const choose = (v: string) => {
    const next = TABS.find((t) => t === v) ?? 'ueberblick';
    setTab(next);
    savePref('balancing.tab', next);
  };
  const edit = (variant?: string) => actions.onEdit(info.spec, variant);
  const wait = (what: string, err?: string) =>
    err ? <NoticeCard title={`${what} nicht möglich`} tone="warn">{err}</NoticeCard> : <Text size="2" color="gray">Lade {what} …</Text>;
  return (
    <article className="bal-report" aria-label="Bericht">
      <DetailHead entry={entry} info={info} summary={summary} actions={actions} />
      {error && <NoticeCard title="Bericht nicht lesbar" tone="error">{error}</NoticeCard>}
      <Tabs.Root value={tab} onValueChange={choose}>
        <Tabs.List aria-label="Bereiche des Berichts">
          <Tabs.Trigger value="ueberblick">Überblick</Tabs.Trigger>
          <Tabs.Trigger value="ziele">Ziele{summary ? ` ${summary.targets.length}` : ''}</Tabs.Trigger>
          <Tabs.Trigger value="diagramme">Diagramme</Tabs.Trigger>
          <Tabs.Trigger value="hinweise">Hinweise{insights ? ` ${insights.hints.length}` : ''}</Tabs.Trigger>
          <Tabs.Trigger value="konfig">Konfiguration</Tabs.Trigger>
        </Tabs.List>
        <div className="bal-tab-body">
          <Tabs.Content value="ueberblick">
            {insights ? <OverviewTab info={info} insights={insights} onTargets={() => choose('ziele')} onEdit={edit} /> : wait('Auswertung', insightsError)}
          </Tabs.Content>
          <Tabs.Content value="ziele">{summary ? <TargetsTab summary={summary} /> : wait('Ziele', error)}</Tabs.Content>
          <Tabs.Content value="diagramme">{summary ? <FiguresTab summary={summary} insights={insights} /> : wait('Diagramme', error)}</Tabs.Content>
          <Tabs.Content value="hinweise">{insights ? <HintsTab insights={insights} onEdit={edit} /> : wait('Hinweise', insightsError)}</Tabs.Content>
          <Tabs.Content value="konfig"><ConfigTab info={info} /></Tabs.Content>
        </div>
      </Tabs.Root>
    </article>
  );
}
