import { Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import type { BalanceReportInfo } from '../api';
import { SpecList } from './SpecList';

/** Konfiguration (B-660): der Auftrag des Laufs als Liste; ältere Berichte haben keinen. */
export function ConfigTab({ info }: { info: BalanceReportInfo }) {
  if (!info.spec) {
    return (
      <NoticeCard title="Älterer Bericht ohne Auftrag" tone="neutral">
        Der Auftrag wurde damals nicht gespeichert. Bekannt: {info.seeds} Seeds{info.varianten?.length ? `, Varianten ${info.varianten.join(', ')}` : ''}.
      </NoticeCard>
    );
  }
  return (
    <section className="bal-card" aria-label="Auftrag">
      <Text weight="bold" size="2">Auftrag</Text>
      <SpecList spec={info.spec} />
    </section>
  );
}
