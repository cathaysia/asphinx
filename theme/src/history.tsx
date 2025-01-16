import { Button } from './components/ui/button';

import { Dialog, DialogContent, DialogTrigger } from '@/components/ui/dialog';
import { useMemo } from 'react';
import useSwr from 'swr';

type CacheType = [string, [string, string, string | null]][];

export function History() {
  const { data } = useSwr<CacheType>(
    '/cache.json',
    (input: RequestInfo | URL, init?: RequestInit) => {
      return fetch(input, init).then(res => res.json());
    },
  );

  const handledData = useMemo(() => {
    if (data === undefined) {
      return undefined;
    }
    const data1 = data.map(item => {
      const [path, [_, title, time]] = item;
      return {
        path: path,
        title: title,
        time: time ? new Date(time) : null,
      };
    });
    data1.sort((a, b) => {
      if (!a.time) {
        return -1;
      }
      if (!b.time) {
        return 1;
      }
      return a.time > b.time ? 1 : -1;
    });
    return data1;
  }, [data]);

  return (
    <Dialog>
      <DialogTrigger>
        <Button variant={'outline'}>History</Button>
      </DialogTrigger>
      <DialogContent className="flex w-full flex-col gap-2 overflow-y-scroll rounded-xl">
        <ul>
          {handledData?.map(item => {
            const time = item.time?.toLocaleString();
            return (
              <li key={item.path}>
                <a href={item.path} className="flex justify-between">
                  <span>{item.title}</span>
                  {time && <div>{`${time}`}</div>}
                </a>
              </li>
            );
          })}
        </ul>
      </DialogContent>
    </Dialog>
  );
}
