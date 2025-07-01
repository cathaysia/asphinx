import {
  ChevronDown,
  ChevronRight,
  File,
  Folder,
  FolderOpen,
  Menu,
  Search,
  X,
} from 'lucide-react';
import { useCallback, useEffect, useState } from 'react';
import { Button } from './ui/button';
import { Input } from './ui/input';
import { ScrollArea } from './ui/scroll-area';

interface FileTreeNode {
  name: string;
  title?: string;
  path: string;
  url?: string;
  is_directory: boolean;
  children: FileTreeNode[];
  level: number;
}

interface FileTreeData {
  root: FileTreeNode[];
  flat_list: FileTreeNode[];
}

interface FileTreeProps {
  currentPath?: string;
}

interface TreeNodeProps {
  node: FileTreeNode;
  isActive: boolean;
  isVisible: boolean;
  onToggle: (path: string) => void;
  expandedNodes: Set<string>;
  searchTerm: string;
  currentPath: string;
}

function TreeNode({
  node,
  isActive,
  isVisible,
  onToggle,
  expandedNodes,
  searchTerm,
  currentPath,
}: TreeNodeProps) {
  const isExpanded = expandedNodes.has(node.path);
  const hasVisibleChildren = node.children.some(child =>
    isNodeVisible(child, searchTerm, expandedNodes),
  );

  if (!isVisible) {
    return null;
  }

  const handleToggle = useCallback(() => {
    if (node.is_directory) {
      onToggle(node.path);
    }
  }, [node.path, node.is_directory, onToggle]);

  const handleClick = useCallback(() => {
    if (node.is_directory) {
      handleToggle();
    } else if (node.url) {
      window.location.href = `/${node.url}`;
    }
  }, [node.is_directory, node.url, handleToggle]);

  const displayName = node.title || node.name.replace('.adoc', '');
  const highlightedName = searchTerm
    ? highlightText(displayName, searchTerm)
    : displayName;

  return (
    <div className="select-none">
      <div
        className={`flex cursor-pointer items-center gap-1 rounded-sm px-2 py-1 text-sm hover:bg-accent hover:text-accent-foreground ${
          isActive ? 'bg-accent font-medium text-accent-foreground' : ''
        }`}
        style={{ paddingLeft: `${node.level * 16 + 8}px` }}
        onClick={handleClick}
      >
        {node.is_directory ? (
          <>
            <Button
              variant="ghost"
              size="sm"
              className="h-4 w-4 p-0 hover:bg-transparent"
              onClick={e => {
                e.stopPropagation();
                handleToggle();
              }}
            >
              {isExpanded ? (
                <ChevronDown className="h-3 w-3" />
              ) : (
                <ChevronRight className="h-3 w-3" />
              )}
            </Button>
            {isExpanded ? (
              <FolderOpen className="h-4 w-4 text-blue-500" />
            ) : (
              <Folder className="h-4 w-4 text-blue-500" />
            )}
          </>
        ) : (
          <>
            <div className="w-4" />
            <File className="h-4 w-4 text-gray-500" />
          </>
        )}
        <span
          className="flex-1 truncate"
          dangerouslySetInnerHTML={{ __html: highlightedName }}
        />
      </div>
      {node.is_directory && isExpanded && hasVisibleChildren && (
        <div>
          {node.children.map(child => (
            <TreeNode
              key={child.path}
              node={child}
              isActive={currentPath === child.url}
              isVisible={isNodeVisible(child, searchTerm, expandedNodes)}
              onToggle={onToggle}
              expandedNodes={expandedNodes}
              searchTerm={searchTerm}
              currentPath={currentPath}
            />
          ))}
        </div>
      )}
    </div>
  );
}

function isNodeVisible(
  node: FileTreeNode,
  searchTerm: string,
  expandedNodes: Set<string>,
): boolean {
  if (!searchTerm) return true;

  const searchLower = searchTerm.toLowerCase();
  const nameMatch = node.name.toLowerCase().includes(searchLower);
  const titleMatch = node.title?.toLowerCase().includes(searchLower) || false;

  if (nameMatch || titleMatch) return true;

  // Check if any children match
  if (node.is_directory) {
    return node.children.some(child =>
      isNodeVisible(child, searchTerm, expandedNodes),
    );
  }

  return false;
}

function highlightText(text: string, searchTerm: string): string {
  if (!searchTerm) return text;

  const regex = new RegExp(`(${searchTerm})`, 'gi');
  return text.replace(
    regex,
    '<mark class="bg-yellow-200 dark:bg-yellow-800">$1</mark>',
  );
}

export default function FileTree({
  currentPath: propCurrentPath,
}: FileTreeProps) {
  const [data, setData] = useState<FileTreeData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(new Set());
  const [isCollapsed, setIsCollapsed] = useState(false);
  const [isMobile, setIsMobile] = useState(false);

  const actualCurrentPath =
    propCurrentPath ||
    (typeof window !== 'undefined'
      ? window.location.pathname.replace(/^\//, '').replace(/\/$/, '') ||
        'index.html'
      : 'index.html');

  // Check if mobile
  useEffect(() => {
    const checkMobile = () => {
      setIsMobile(window.innerWidth < 768);
    };

    checkMobile();
    window.addEventListener('resize', checkMobile);
    return () => window.removeEventListener('resize', checkMobile);
  }, []);

  // Load file tree data
  useEffect(() => {
    const loadData = async () => {
      try {
        const response = await fetch('/filetree.json');
        if (!response.ok) {
          throw new Error('Failed to load file tree data');
        }
        const treeData: FileTreeData = await response.json();
        setData(treeData);

        // Auto-expand path to current file
        if (actualCurrentPath) {
          const pathParts = actualCurrentPath.replace('.html', '').split('/');
          const newExpanded = new Set<string>();
          let currentDir = '';

          for (let i = 0; i < pathParts.length - 1; i++) {
            currentDir = currentDir
              ? `${currentDir}/${pathParts[i]}`
              : pathParts[i];
            newExpanded.add(currentDir);
          }

          setExpandedNodes(newExpanded);
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Unknown error');
      } finally {
        setLoading(false);
      }
    };

    loadData();
  }, [actualCurrentPath]);

  // Load/save expanded state
  useEffect(() => {
    const saved = localStorage.getItem('filetree-expanded');
    if (saved) {
      try {
        const savedArray: string[] = JSON.parse(saved);
        const savedSet = new Set(savedArray);
        setExpandedNodes(prev => new Set([...prev, ...savedSet]));
      } catch (e) {
        // Ignore invalid saved state
      }
    }
  }, []);

  useEffect(() => {
    localStorage.setItem(
      'filetree-expanded',
      JSON.stringify([...expandedNodes]),
    );
  }, [expandedNodes]);

  // Load/save collapsed state
  useEffect(() => {
    const saved = localStorage.getItem('filetree-collapsed');
    if (saved === 'true') {
      setIsCollapsed(true);
    }
  }, []);

  useEffect(() => {
    localStorage.setItem('filetree-collapsed', isCollapsed.toString());
  }, [isCollapsed]);

  const handleToggle = useCallback((path: string) => {
    setExpandedNodes(prev => {
      const newSet = new Set(prev);
      if (newSet.has(path)) {
        newSet.delete(path);
      } else {
        newSet.add(path);
      }
      return newSet;
    });
  }, []);

  const handleSearch = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const term = e.target.value;
      setSearchTerm(term);

      // Auto-expand nodes when searching
      if (term && data) {
        const newExpanded = new Set(expandedNodes);
        data.flat_list.forEach(node => {
          if (node.is_directory && isNodeVisible(node, term, expandedNodes)) {
            newExpanded.add(node.path);
          }
        });
        setExpandedNodes(newExpanded);
      }
    },
    [data, expandedNodes],
  );

  const toggleCollapse = useCallback(() => {
    setIsCollapsed(prev => !prev);
  }, []);

  if (loading) {
    return (
      <div className="p-4">
        <div className="animate-pulse space-y-2">
          <div className="h-4 w-3/4 rounded bg-gray-200" />
          <div className="h-4 w-1/2 rounded bg-gray-200" />
          <div className="h-4 w-2/3 rounded bg-gray-200" />
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-4 text-red-500 text-sm">
        Error loading file tree: {error}
      </div>
    );
  }

  if (!data) {
    return null;
  }

  const toggleButton = (
    <Button
      variant="ghost"
      size="sm"
      onClick={toggleCollapse}
      className="h-8 w-8 p-0"
      title={isCollapsed ? 'Show file tree' : 'Hide file tree'}
    >
      {isCollapsed ? <Menu className="h-4 w-4" /> : <X className="h-4 w-4" />}
    </Button>
  );

  if (isCollapsed) {
    return (
      <div
        className={`${isMobile ? 'mb-4' : 'w-8'} flex ${isMobile ? 'justify-start' : 'flex-col'}`}
      >
        {toggleButton}
      </div>
    );
  }

  return (
    <div className={`${isMobile ? 'mb-4' : 'w-64'} border-r bg-background`}>
      <div className="border-b p-3">
        <div className="mb-3 flex items-center justify-between">
          <h3 className="font-semibold text-sm">Contents</h3>
          {toggleButton}
        </div>
        <div className="relative">
          <Search className="absolute top-2.5 left-2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Search..."
            value={searchTerm}
            onChange={handleSearch}
            className="h-8 pl-8 text-sm"
          />
        </div>
      </div>
      <ScrollArea className={`${isMobile ? 'h-64' : 'h-[calc(100vh-12rem)]'}`}>
        <div className="p-2">
          {data.root.map(node => (
            <TreeNode
              key={node.path}
              node={node}
              isActive={actualCurrentPath === node.url}
              isVisible={isNodeVisible(node, searchTerm, expandedNodes)}
              onToggle={handleToggle}
              expandedNodes={expandedNodes}
              searchTerm={searchTerm}
              currentPath={actualCurrentPath}
            />
          ))}
        </div>
      </ScrollArea>
    </div>
  );
}
