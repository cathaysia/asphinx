import { History } from '@/history';
import SearchBar from '@/search-bar';
import { Moon, Sun } from 'lucide-react';
import { useState } from 'react';
import reactDom from 'react-dom/client';

import renderMathInElement from 'katex/contrib/auto-render';
import { Button } from './components/ui/button';

renderMathInElement(document.body);

function ThemeButton() {
  document.documentElement.classList.toggle(
    'dark',
    localStorage.theme === 'dark',
  );
  const [isDark, setIsdark] = useState(localStorage.them === 'dark');

  return (
    <Button
      variant={'ghost'}
      onClick={() => {
        localStorage.theme = localStorage.theme === 'dark' ? 'light' : 'dark';

        document.documentElement.classList.toggle(
          'dark',
          localStorage.theme === 'dark',
        );
        setIsdark(localStorage.theme === 'dark');
      }}
    >
      {isDark ? (
        <Moon className="h-[1.2rem] w-[1.2rem]" />
      ) : (
        <Sun className="h-[1.2rem] w-[1.2rem]" />
      )}
    </Button>
  );
}

const themeBtn = document.getElementById('theme_btn');
// biome-ignore lint/style/noNonNullAssertion:
const btn = reactDom.createRoot(themeBtn!);

btn.render(<ThemeButton />);

const searchBar = document.getElementById('search_bar');
// biome-ignore lint/style/noNonNullAssertion:
const bar = reactDom.createRoot(searchBar!);

bar.render(<SearchBar />);

const history = document.getElementById('history');
// biome-ignore lint/style/noNonNullAssertion:
const his = reactDom.createRoot(history!);

his.render(<History />);
