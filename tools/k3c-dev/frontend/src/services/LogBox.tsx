import { Flex } from '@radix-ui/themes';
import { useCallback, useEffect, useState } from 'react';
import { backend, type LevelCounts } from '../api';
import { errorText } from '../lib/errors';
import { formatNumber } from '../lib/format';
import { ActionButton } from '../ui/parts';
import { levelShares } from './tables';

/** Kasten `LOG · LETZTE 60 MINUTEN` einer Karte (nur für Dienste mit Log). */
export function LogBox({ name }: { name: string }) {
  const [counts, setCounts] = useState<LevelCounts | null>(null);
  const [error, setError] = useState('');
  const load = useCallback(async () => {
    try {
      setCounts(await backend.serviceLogLevels(name));
      setError('');
    } catch (e) {
      setError(errorText(e));
    }
  }, [name]);
  useEffect(() => void load(), [load]);

  const shares = counts ? levelShares(counts) : [];
  return (
    <div className="svc-log">
      <Flex justify="between" align="center">
        <span className="svc-label">LOG · LETZTE 60 MINUTEN</span>
        <ActionButton size="1" variant="ghost" onClick={load}>Aktualisieren</ActionButton>
      </Flex>
      {error && <p className="svc-error">{error}</p>}
      {counts && (
        <>
          <div className="svc-log-total">
            {formatNumber(counts.total)} Einträge{counts.budgetHit && ' · ältere nicht gelesen'}
          </div>
          <div className="svc-bar" aria-hidden>
            {shares.filter((x) => x.count > 0).map((x) => (
              <span key={x.level} className={`lvl-${x.level}`} style={{ width: `${x.percent}%` }} />
            ))}
          </div>
          <Flex gap="3" wrap="wrap" className="svc-levels">
            {shares.map((x) => (
              <span key={x.level} className={x.level === 'ERROR' && x.count > 0 ? 'svc-error-count' : undefined}>
                {x.level} {formatNumber(x.count)}
              </span>
            ))}
          </Flex>
        </>
      )}
    </div>
  );
}
