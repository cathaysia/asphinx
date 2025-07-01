import FileTree from '@/components/file-tree';
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

// Initialize file tree for desktop
const fileTreeDesktop = document.getElementById('file_tree_desktop');
if (fileTreeDesktop) {
  const desktopTree = reactDom.createRoot(fileTreeDesktop);
  desktopTree.render(<FileTree />);
}

// Initialize file tree for mobile
const fileTreeMobile = document.getElementById('file_tree_mobile');
if (fileTreeMobile) {
  const mobileTree = reactDom.createRoot(fileTreeMobile);
  mobileTree.render(<FileTree />);
}
