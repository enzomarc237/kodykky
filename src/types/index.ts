// Core types for the application
export interface ProjectFile {
  path: string;
  content: string;
  isDirectory: boolean;
  size?: number;
}

export interface ScaffoldRequest {
  prompt: string;
  framework?: string;
}

export interface ScaffoldResponse {
  framework: string;
  project_name: string;
  structure: ProjectStructure[];
  files: Record<string, string>;
  dependencies: Record<string, string>;
}

export interface ProjectStructure {
  path: string;
  is_directory: boolean;
  size?: number;
}

export interface ProjectHistory {
  id: string;
  name: string;
  prompt: string;
  framework: string;
  path: string;
  created_at: string;
}

export interface AppSettings {
  apiKey?: string;
  defaultProjectLocation?: string;
  preferredEditor: 'vscode' | 'vscode-insiders' | 'default';
  theme: 'dark' | 'light' | 'system';
  autoSaveHistory: boolean;
  maxHistoryItems: number;
  confirmBeforeOverwrite: boolean;
}

export interface AppState {
  // Current scaffold generation
  isGenerating: boolean;
  currentScaffold?: ScaffoldResponse;
  generationError?: string;
  
  // Project history
  projectHistory: ProjectHistory[];
  
  // Settings
  settings: AppSettings;
  
  // UI state
  selectedFramework?: string;
  previewMode: 'tree' | 'code';
  selectedFile?: string;
  isSettingsOpen: boolean;
}

export type Framework = 
  | 'react'
  | 'nextjs'
  | 'flask'
  | 'express'
  | 'cli';

export interface FrameworkInfo {
  id: Framework;
  name: string;
  description: string;
  icon: string;
  color: string;
}

export const SUPPORTED_FRAMEWORKS: FrameworkInfo[] = [
  {
    id: 'react',
    name: 'React + Vite',
    description: 'Modern React with Vite build tool',
    icon: '⚛️',
    color: '#61DAFB'
  },
  {
    id: 'nextjs',
    name: 'Next.js',
    description: 'Full-stack React framework',
    icon: '▲',
    color: '#000000'
  },
  {
    id: 'flask',
    name: 'Flask',
    description: 'Python web framework',
    icon: '🐍',
    color: '#3776AB'
  },
  {
    id: 'express',
    name: 'Express',
    description: 'Node.js web framework',
    icon: '🚀',
    color: '#339999'
  },
  {
    id: 'cli',
    name: 'CLI Tool',
    description: 'Node.js command-line tool',
    icon: '💻',
    color: '#68A063'
  }
];