import Fuse, { type FuseResult } from 'fuse.js';
import { useEffect, useMemo, useState } from 'react';
import { Button } from './components/ui/button';

import { Dialog, DialogContent, DialogTrigger } from '@/components/ui/dialog';
import useSwr from 'swr';
import { useDebounce } from 'use-debounce';
import { Input } from './components/ui/input';
import { Label } from './components/ui/label';
import { ScrollArea } from './components/ui/scroll-area';

type CacheType = [string, [string, string]][];

export default function SearchBar() {
  const [search, setSearch] = useState('');
  const [debounce] = useDebounce(search, 200);
  const [result, setResult] =
    useState<
      FuseResult<{
        file: string;
        content: [string, string];
      }>[]
    >();
  const { data } = useSwr<CacheType>(
    '/cache.json',
    (input: RequestInfo | URL, init?: RequestInit) => {
      return fetch(input, init).then(res => res.json());
    },
  );
  const fuse = useMemo(() => {
    if (!data) {
      return;
    }
    const v = data as CacheType;
    const posts = v.map(item => {
      return {
        file: item[0],
        content: item[1],
      };
    });
    return new Fuse(posts, {
      keys: ['file', 'content'],
    });
  }, [data]);

  useEffect(() => {
    if (!data || !fuse) {
      return;
    }
    if (debounce.length === 0) {
      setResult(undefined);
      return;
    }

    const res = fuse.search(debounce);
    setResult(res);
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
          <div className="flex w-[304px] flex-col gap-2 sm:w-[520px]">
            {result
              ? result.map(item => {
                  return (
                    <a
                      key={item.item.file}
                      className="flex w-full flex-col items-start rounded border p-2 shadow"
                      href={`/${item.item.file}`}
                    >
                      <Label className="w-full min-w-0 overflow-x-hidden text-ellipsis whitespace-nowrap text-lg">
                        {item.item.content[1] || item.item.file}
                      </Label>
                      <Label className="w-full min-w-0 overflow-x-hidden text-ellipsis whitespace-nowrap text-gray-600 text-sm">
                        {item.item.content[0]}
                      </Label>
                    </a>
                  );
                })
              : data?.map(item => {
                  return (
                    <div
                      key={item[0]}
                      className="flex h-[74px] items-center rounded border p-2 shadow "
                    >
                      <a
                        href={`/${item[0]}`}
                        className="min-w-0 overflow-x-hidden text-ellipsis whitespace-nowrap"
                      >
                        {item[1][1]}
                      </a>
                    </div>
                  );
                })}
          </div>
        </ScrollArea>
        {result ? (
          <Label className="text-muted-foreground text-sm">
            total {result?.length}
          </Label>
        ) : (
          <Label className="text-muted-foreground text-sm">&nbsp;</Label>
        )}
      </DialogContent>
    </Dialog>
  );
}
