import { Flex, Text } from '@webkit/atoms';
import { LineChart, NoticeCard } from '@webkit/molecules';
import type { PlaytestStats } from './types';

/** Bytes als „KB“ oder „MB“. */
export function size(bytes: number): string {
  return bytes >= 1024 * 1024 ? `${(bytes / 1024 / 1024).toFixed(1)} MB` : `${Math.round(bytes / 1024)} KB`;
}

const kbps = (rates: number[]) => rates.map((v, i) => ({ x: i - rates.length + 1, y: Math.round(v / 1024) }));

function Chart({ name, rates: raw, total }: { name: string; rates: number[] | null; total: number }) {
  const rates = raw ?? []; // vor dem ersten Messtakt kann die Liste fehlen
  const last = rates.length ? rates[rates.length - 1] : 0;
  return (
    <Flex direction="column" gap="1" flexGrow="1" style={{ minWidth: 220 }} data-chart={name}>
      <Text size="2" weight="bold">{name}: {Math.round(last / 1024)} KB/s · gesamt {size(total)}</Text>
      <LineChart series={[{ name, points: kbps(rates) }]} xLabel="Sekunden" yLabel="KB/s" height={130} />
    </Flex>
  );
}

/** Zwei kleine Diagramme während der Aufnahme (Ton und Bild: was je Sekunde ankommt) und die Warnung, wenn etwas steht. */
export function RecordMonitor({ stats }: { stats: PlaytestStats }) {
  return (
    <Flex direction="column" gap="2" data-monitor={stats.warn ? 'warn' : 'ok'}>
      {stats.warn && <NoticeCard title="Aufnahme prüfen" tone="error">{stats.warn}</NoticeCard>}
      <Flex gap="4" wrap="wrap">
        <Chart name="Ton" rates={stats.voiceRate} total={stats.voiceBytes} />
        {!stats.noVideo && <Chart name="Bild" rates={stats.videoRate} total={stats.videoBytes} />}
      </Flex>
    </Flex>
  );
}
