import { Button } from './components/ui/button';

import { Dialog, DialogContent, DialogTrigger } from '@/components/ui/dialog';
import { useMemo, useState } from 'react';
import useSwr from 'swr';
import { useDebounce } from 'use-debounce';
import { Input } from './components/ui/input';
import { ScrollArea } from './components/ui/scroll-area';
import { Skeleton } from './components/ui/skeleton';

type CacheType = [string, [string, string, string | null]][];

export function History() {
  const [search, setSearch] = useState('');
  const [debounce] = useDebounce(search, 200);
  const { data, isLoading } = useSwr<CacheType>(
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
          {isLoading && (
            <div className="flex max-w-[270px] flex-col gap-2 md:max-w-[460px]">
              <Skeleton className="h-16 w-full" />
              <Skeleton className="h-16 w-full" />
              <Skeleton className="h-16 w-full" />
              <Skeleton className="h-16 w-full" />
              <Skeleton className="h-16 w-full" />
              <Skeleton className="h-16 w-full" />
            </div>
          )}
          {!isLoading && (
            <ul className="flex max-w-[270px] flex-col gap-2 md:max-w-[460px]">
              {handledData
                ?.filter(item => {
                  if (debounce === '') {
                    return true;
                  }
                  return item.title.toLowerCase().includes(debounce);
                })
                .map(item => {
                  let time = null;
                  let fulltime = undefined;
                  if (item.time) {
                    time = dateInYyyyMmDd(item.time);
                    fulltime = dateInYyyyMmDdHhMmSs(item.time);
                  }
                  return (
                    <li key={item.path}>
                      <a
                        href={`/${item.path}`}
                        className="flex w-full justify-between gap-2"
                      >
                        <span className="min-w-0 overflow-hidden text-ellipsis whitespace-nowrap">
                          {item.title}
                        </span>
                        {time && (
                          <span
                            className="whitespace-nowrap font-mono"
                            title={fulltime}
                          >{`${time}`}</span>
                        )}
                      </a>
                    </li>
                  );
                })}
            </ul>
          )}
        </ScrollArea>
      </DialogContent>
    </Dialog>
  );
}

function padTwoDigits(num: number) {
  return num.toString().padStart(2, '0');
}

function dateInYyyyMmDdHhMmSs(date: Date) {
  // :::: Exmple Usage ::::
  // The function takes a Date object as a parameter and formats the date as YYYY-MM-DD hh:mm:ss.
  // 👇 2023-04-11 16:21:23 (yyyy-mm-dd hh:mm:ss)
  //console.log(dateInYyyyMmDdHhMmSs(new Date()));

  //  👇 2025-05-04 05:24:07 (yyyy-mm-dd hh:mm:ss)
  // console.log(dateInYyyyMmDdHhMmSs(new Date('May 04, 2025 05:24:07')));
  // Date divider
  // 👇 01/04/2023 10:20:07 (MM/DD/YYYY hh:mm:ss)
  // console.log(dateInYyyyMmDdHhMmSs(new Date(), "/"));
  return `${date.getFullYear()}/${padTwoDigits(date.getMonth() + 1)}/${padTwoDigits(date.getDate())} ${padTwoDigits(date.getHours())}:${padTwoDigits(date.getMinutes())}:${padTwoDigits(date.getSeconds())}`;
}

function dateInYyyyMmDd(date: Date) {
  // :::: Exmple Usage ::::
  // The function takes a Date object as a parameter and formats the date as YYYY-MM-DD hh:mm:ss.
  // 👇 2023-04-11 16:21:23 (yyyy-mm-dd hh:mm:ss)
  //console.log(dateInYyyyMmDdHhMmSs(new Date()));

  //  👇 2025-05-04 05:24:07 (yyyy-mm-dd hh:mm:ss)
  // console.log(dateInYyyyMmDdHhMmSs(new Date('May 04, 2025 05:24:07')));
  // Date divider
  // 👇 01/04/2023 10:20:07 (MM/DD/YYYY hh:mm:ss)
  // console.log(dateInYyyyMmDdHhMmSs(new Date(), "/"));
  return `${date.getFullYear()}/${padTwoDigits(date.getMonth() + 1)}/${padTwoDigits(date.getDate())}`;
}
