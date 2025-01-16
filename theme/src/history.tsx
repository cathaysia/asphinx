import { Button } from './components/ui/button';

import { Dialog, DialogContent, DialogTrigger } from '@/components/ui/dialog';
import { useMemo, useState } from 'react';
import useSwr from 'swr';
import { useDebounce } from 'use-debounce';
import { Input } from './components/ui/input';
import { ScrollArea } from './components/ui/scroll-area';

type CacheType = [string, [string, string, string | null]][];

export function History() {
  const [search, setSearch] = useState('');
  const [debounce] = useDebounce(search, 200);
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
        return 1;
      }
      if (!b.time) {
        return -1;
      }
      return a.time > b.time ? -1 : 1;
    });
    return data1;
  }, [data]);

  return (
    <Dialog>
      <DialogTrigger>
        <Button variant={'outline'}>History</Button>
      </DialogTrigger>
      <DialogContent className="flex w-full flex-col gap-2 rounded-xl">
        <Input
          placeholder="type for search"
          value={search}
          onChange={e => setSearch(e.target.value)}
        />
        <ScrollArea className="h-[320px]">
          <ul>
            {handledData
              ?.filter(item => {
                if (debounce === '') {
                  return true;
                }
                return item.title.includes(debounce);
              })
              .map(item => {
                const time = item.time?.toLocaleString();
                return (
                  <li key={item.path}>
                    <a href={`/${item.path}`} className="flex justify-between">
                      <span>{item.title}</span>
                      {time && <div>{`${time}`}</div>}
                    </a>
                  </li>
                );
              })}
          </ul>
        </ScrollArea>
      </DialogContent>
    </Dialog>
  );
}
