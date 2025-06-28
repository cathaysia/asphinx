import { Dialog, DialogContent, DialogTrigger } from '@/components/ui/dialog';
import { useCallback, useEffect, useState } from 'react';
import { useDebounce } from 'use-debounce';
import { Button } from './components/ui/button';
import { Input } from './components/ui/input';
import { Label } from './components/ui/label';
import { ScrollArea } from './components/ui/scroll-area';
import { Skeleton } from './components/ui/skeleton';

interface SearchResultItem {
  path: string;
  content: string;
  title: string;
  time: Date | null;
}

export default function SearchBar() {
  const [search, setSearch] = useState('');
  const [debouncedSearch] = useDebounce(search, 300);
  const [results, setResults] = useState<SearchResultItem[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [pagefind, setPagefind] = useState<PagefindInstance | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const initPagefind = async () => {
      try {
        // eslint-disable-next-line @typescript-eslint/no-implied-eval
        const importPagefind = new Function(
          'return import("/pagefind/pagefind.js")',
        );
        const pagefindModule = await importPagefind();

        const pf = pagefindModule as PagefindInstance;

        await pf.init();
        setPagefind(pf);
      } catch (err) {
        console.error('Failed to initialize Pagefind:', err);
        setError('search initialization failed');
      }
    };

    initPagefind();
  }, []);

  const convertPagefindResult = useCallback(
    (fragment: PagefindSearchFragment): SearchResultItem => {
      const extractDate = (meta: Record<string, string>): Date | null => {
        if (meta.date) {
          const date = new Date(meta.date);
          return Number.isNaN(date.getTime()) ? null : date;
        }
        return null;
      };

      return {
        path: fragment.url.replace(/^\//, ''),
        content: fragment.excerpt.replace(/<\/?mark>/g, ''),
        title:
          fragment.meta.title ||
          fragment.url.split('/').pop()?.replace('.html', '') ||
          'Untitled',
        time: extractDate(fragment.meta),
      };
    },
    [],
  );

  const getDefaultResults = useCallback(async (): Promise<
    SearchResultItem[]
  > => {
    if (!pagefind) {
      return [];
    }

    try {
      const searchResult = await pagefind.search('the', {});

      if (searchResult.results.length === 0) {
        return [];
      }

      const fragments = await Promise.all(
        searchResult.results.slice(0, 10).map(result => result.data()),
      );

      const convertedResults = fragments.map(convertPagefindResult);

      convertedResults.sort((a, b) => {
        if (!a.time && !b.time) return 0;
        if (!a.time) return 1;
        if (!b.time) return -1;
        return b.time.getTime() - a.time.getTime();
      });

      return convertedResults;
    } catch (err) {
      console.error('Failed to get default results:', err);
      return [];
    }
  }, [pagefind, convertPagefindResult]);

  const performSearch = useCallback(
    async (query: string) => {
      if (!pagefind) {
        return;
      }

      if (!query.trim()) {
        setIsLoading(true);
        setError(null);
        const defaultResults = await getDefaultResults();
        setResults(defaultResults);
        setIsLoading(false);
        return;
      }

      setIsLoading(true);
      setError(null);

      try {
        pagefind.preload(query);

        const searchResult = await pagefind.search(query, {});

        if (searchResult.results.length === 0) {
          const defaultResults = await getDefaultResults();
          setResults(defaultResults);
          setIsLoading(false);
          return;
        }

        const fragments = await Promise.all(
          searchResult.results.slice(0, 10).map(result => result.data()),
        );

        const convertedResults = fragments.map(convertPagefindResult);

        convertedResults.sort((a, b) => {
          if (!a.time && !b.time) return 0;
          if (!a.time) return 1;
          if (!b.time) return -1;
          return b.time.getTime() - a.time.getTime();
        });

        setResults(convertedResults);
      } catch (err) {
        console.error('Search failed:', err);
        setError('search failed, please try again later');
        setResults([]);
      } finally {
        setIsLoading(false);
      }
    },
    [pagefind, convertPagefindResult, getDefaultResults],
  );

  useEffect(() => {
    performSearch(debouncedSearch);
  }, [debouncedSearch, performSearch]);

  const handleSearchChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      setSearch(e.target.value);
    },
    [],
  );

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
          type="search"
          value={search}
          onChange={handleSearchChange}
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
          {error && (
            <div className="flex max-w-[270px] flex-col gap-2 md:max-w-[460px]">
              <div className="p-2 text-red-500 text-sm">{error}</div>
            </div>
          )}
          {!isLoading && !error && (
            <div className="flex max-w-[270px] flex-col gap-2 md:max-w-[460px]">
              {results.map((item, index) => (
                <div key={`${item.path}-${index}`} className="w-full">
                  <a
                    className="flex w-full flex-col items-start rounded border p-2 shadow"
                    href={`/${item.path}`}
                  >
                    <Label className="w-full">
                      <div className="flex min-w-0 justify-between overflow-x-hidden text-ellipsis whitespace-nowrap text-lg">
                        <span>{item.title}</span>
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
              ))}
            </div>
          )}
        </ScrollArea>
        {!isLoading && !error && (
          <Label className="font-mono text-muted-foreground text-sm">
            total {results.length}
          </Label>
        )}
        {isLoading && (
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
  return `${date.getFullYear()}/${padTwoDigits(date.getMonth() + 1)}/${padTwoDigits(date.getDate())} ${padTwoDigits(date.getHours())}:${padTwoDigits(date.getMinutes())}:${padTwoDigits(date.getSeconds())}`;
}

export function dateInYyyyMmDd(date: Date) {
  return `${date.getFullYear()}/${padTwoDigits(date.getMonth() + 1)}/${padTwoDigits(date.getDate())}`;
}
