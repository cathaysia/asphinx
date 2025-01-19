import Fuse from 'fuse.js';
import { useMemo, useState } from 'react';
import { Button } from './components/ui/button';

import { Dialog, DialogContent, DialogTrigger } from '@/components/ui/dialog';
import useSwr from 'swr';
import { useDebounce } from 'use-debounce';
import { Input } from './components/ui/input';
import { Label } from './components/ui/label';
import { ScrollArea } from './components/ui/scroll-area';
import { Skeleton } from './components/ui/skeleton';
import { dateInYyyyMmDd, dateInYyyyMmDdHhMmSs } from './history';

type CacheType = [string, [string, string, string | null]][];

export default function SearchBar() {
  const [search, setSearch] = useState('');
  const [debounce] = useDebounce(search, 200);
  const { data, error, isLoading } = useSwr<CacheType>(
    '/cache.json',
    async (input: RequestInfo | URL, init?: RequestInit) => {
      const data = await fetch(input, init);
      return data.json();
    },
  );
  const fuse = useMemo(() => {
    if (!data) {
      return;
    }
    const v = data as CacheType;
    const posts = v.map(item => {
      const [path, [content, title, time]] = item;
      return {
        path: path,
        content: content,
        title: title,
        time: time ? new Date(time) : null,
      };
    });
    return new Fuse(posts, {
      keys: ['file', 'content'],
    });
  }, [data]);

  const result = useMemo(() => {
    if (!data || !fuse) {
      return null;
    }
    if (debounce.length === 0) {
      const data1 = data.map(item => {
        const [path, [content, title, time]] = item;
        return {
          item: {
            path: path,
            content: content,
            title: title,
            time: time ? new Date(time) : null,
          },
        };
      });

      data1.sort((a, b) => {
        if (!a.item.time) {
          return 1;
        }
        if (!b.item.time) {
          return -1;
        }
        return a.item.time > b.item.time ? -1 : 1;
      });
      return data1;
    }
    const res = fuse.search(debounce);
    return res;
  }, [data, debounce]);

  return (
    <Dialog>
      <DialogTrigger>
        <Button
          variant={'outline'}
          className="relative inline-flex h-8 w-full items-center justify-start gap-2 whitespace-nowrap rounded-[0.5rem] border border-input bg-muted/50 px-4 py-2 font-normal text-muted-foreground text-sm shadow-none transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 sm:pr-12 md:w-40 lg:w-56 xl:w-64 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
        >
          Search
        </Button>
      </DialogTrigger>
      <DialogContent className="flex w-80 flex-col gap-2 rounded-xl sm:w-full">
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
          {!isLoading && !error && (
            <div className="flex max-w-[270px] flex-col gap-2 md:max-w-[460px]">
              {result?.map(item => {
                return (
                  <div key={item.item.path} className="w-full">
                    <a
                      className="flex w-full flex-col items-start rounded border p-2 shadow"
                      href={`/${item.item.path}`}
                    >
                      <Label className="w-full">
                        <div className="flex min-w-0 justify-between overflow-x-hidden text-ellipsis whitespace-nowrap text-lg">
                          <span>{item.item.title || item.item.path}</span>
                          <span
                            className="font-mono font-normal"
                            title={
                              item.item.time
                                ? dateInYyyyMmDdHhMmSs(item.item.time)
                                : undefined
                            }
                          >
                            {item.item.time && dateInYyyyMmDd(item.item.time)}
                          </span>
                        </div>
                      </Label>
                      <Label className="line-clamp-2 w-full min-w-0 text-gray-600 text-sm">
                        {item.item.content}
                      </Label>
                    </a>
                  </div>
                );
              })}
            </div>
          )}
        </ScrollArea>
        {result ? (
          <Label className="font-mono text-muted-foreground text-sm">
            total {result?.length}
          </Label>
        ) : (
          <Label className="text-muted-foreground text-sm">&nbsp;</Label>
        )}
      </DialogContent>
    </Dialog>
  );
}
