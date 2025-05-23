import MiniSearch from 'minisearch';
import { useMemo, useState } from 'react';
import { Button } from './components/ui/button';

import { Dialog, DialogContent, DialogTrigger } from '@/components/ui/dialog';
import useSwr from 'swr';
import { useDebounce } from 'use-debounce';
import { Input } from './components/ui/input';
import { Label } from './components/ui/label';
import { ScrollArea } from './components/ui/scroll-area';
import { Skeleton } from './components/ui/skeleton';

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
  const miniSearch = useMemo(() => {
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

    const miniSearch = new MiniSearch<{
      path: string;
      content: string;
      title: string;
      time: Date | null;
    }>({
      fields: ['title', 'content'],
      idField: 'path',
      storeFields: ['path', 'content', 'title', 'time'],
      tokenize: text => {
        text = text.toLowerCase();
        // TODO: better CJK tokenizer
        // NOTE: How to inject dependency (n-gram etc.) into here? `tokenize` will ignore top-level import somehow,
        // and it can't be made async which means we can't dynamic import.
        const segmenter =
          Intl.Segmenter && new Intl.Segmenter('zh', { granularity: 'word' });
        if (!segmenter) return [text]; // firefox?
        return Array.from(segmenter.segment(text), ({ segment }) => segment);
      },
    });
    miniSearch.addAll(posts);
    return miniSearch;
  }, [data]);

  const result = useMemo(() => {
    if (!data || !miniSearch) {
      return null;
    }
    if (debounce.length === 0) {
      const data1 = data.map(item => {
        const [path, [content, title, time]] = item;
        return {
          path: path,
          content: content,
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
    }
    const res = miniSearch.search(debounce, {
      combineWith: 'AND',
      // don't split search word, user searching "泛函" shouldn't get "广泛" or "函数"
      // XXX: This is a hack, we should probably use a better CJK tokenizer
      tokenize: text => [text.toLowerCase()],
      fuzzy(term) {
        // disable fuzzy search if the term contains a CJK character
        // so searching "函数式" will not contain results only matching "函数"
        const cjkRange =
          '\u2e80-\u2eff\u2f00-\u2fdf\u3040-\u309f\u30a0-\u30fa\u30fc-\u30ff\u3100-\u312f\u3200-\u32ff\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff';
        const cjkWord = new RegExp(`[${cjkRange}]`);
        if (cjkWord.test(term)) return false;
        return true;
      },
    });
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
                  <div key={item.path} className="w-full">
                    <a
                      className="flex w-full flex-col items-start rounded border p-2 shadow"
                      href={`/${item.path}`}
                    >
                      <Label className="w-full">
                        <div className="flex min-w-0 justify-between overflow-x-hidden text-ellipsis whitespace-nowrap text-lg">
                          <span>{item.title || item.path}</span>
                          <span
                            className="font-mono font-normal"
                            title={
                              item.time
                                ? dateInYyyyMmDdHhMmSs(item.time)
                                : undefined
                            }
                          >
                            {item.time && dateInYyyyMmDd(item.time)}
                          </span>
                        </div>
                      </Label>
                      <Label className="line-clamp-2 w-full min-w-0 text-gray-600 text-sm">
                        {item.content}
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

function padTwoDigits(num: number) {
  return num.toString().padStart(2, '0');
}

export function dateInYyyyMmDdHhMmSs(date: Date) {
  // :::: Example Usage ::::
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

export function dateInYyyyMmDd(date: Date) {
  // :::: Example Usage ::::
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
