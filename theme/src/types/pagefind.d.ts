declare global {
  type PagefindSearchResults = {
    results: PagefindSearchResult[];
    unfilteredResultCount: number;
    filters: Record<string, Record<string, number>>;
    totalFilters: Record<string, Record<string, number>>;
    timings: {
      preload: number;
      search: number;
      total: number;
    };
  };

  type PagefindSearchResult = {
    id: string;
    score: number;
    words: number[];
    data: () => Promise<PagefindSearchFragment>;
  };

  type PagefindSearchFragment = {
    url: string;
    raw_url?: string;
    content: string;
    raw_content?: string;
    excerpt: string;
    sub_results: PagefindSubResult[];
    word_count: number;
    locations: number[];
    weighted_locations: PagefindWordLocation[];
    filters: Record<string, string[]>;
    meta: Record<string, string>;
    anchors: PagefindSearchAnchor[];
  };

  type PagefindSubResult = {
    title: string;
    url: string;
    locations: number[];
    weighted_locations: PagefindWordLocation[];
    excerpt: string;
    anchor?: PagefindSearchAnchor;
  };

  type PagefindWordLocation = {
    weight: number;
    balanced_score: number;
    location: number;
  };

  type PagefindSearchAnchor = {
    element: string;
    id: string;
    text?: string;
    location: number;
  };

  interface Window {
    pagefind?: PagefindInstance;
  }

  interface PagefindInstance {
    init(): Promise<void>;
    search(
      query: string,
      options?: Record<string, unknown>,
    ): Promise<PagefindSearchResults>;
    preload(query: string, options?: Record<string, unknown>): void;
    destroy(): Promise<void>;
    options(opts: Record<string, unknown>): Promise<void>;
  }
}

export type {};
