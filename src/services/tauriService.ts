import { invoke } from '@tauri-apps/api/core';
import type { ScaffoldRequest, ScaffoldResponse, ProjectHistory } from '../types';

class TauriService {
  // AI Service
  async generateScaffold(request: ScaffoldRequest, apiKey: string): Promise<ScaffoldResponse> {
    return invoke('generate_scaffold', {
      prompt: request.prompt,
      framework: request.framework,
      apiKey,
    });
  }

  async validateApiKey(apiKey: string): Promise<boolean> {
    return invoke('validate_api_key', { apiKey });
  }

  async saveApiKey(apiKey: string): Promise<void> {
    return invoke('save_api_key', { apiKey });
  }

  async getApiKey(): Promise<string | null> {
    return invoke('get_api_key');
  }

  // File System Service
  async createProject(path: string, structure: any): Promise<void> {
    return invoke('create_project', { path, structure });
  }

  async validateProjectPath(path: string): Promise<boolean> {
    return invoke('validate_project_path', { path });
  }

  async selectDirectory(): Promise<string | null> {
    return invoke('select_directory');
  }

  // Editor Service
  async openInVscode(projectPath: string): Promise<void> {
    return invoke('open_in_vscode', { projectPath });
  }

  async openInFinder(path: string): Promise<void> {
    return invoke('open_in_finder', { path });
  }

  // Storage Service
  async getProjectHistory(): Promise<ProjectHistory[]> {
    return invoke('get_project_history');
  }

  async addToHistory(project: ProjectHistory): Promise<void> {
    return invoke('add_to_history', { project });
  }

  async deleteFromHistory(id: string): Promise<void> {
    return invoke('delete_from_history', { id });
  }
}

export const tauriService = new TauriService();

// Utility functions that don't require Tauri
export const fileUtils = {
  downloadFile: (content: string, filename: string) => {
    const blob = new Blob([content], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);
  },

  copyToClipboard: async (text: string) => {
    if (navigator.clipboard) {
      await navigator.clipboard.writeText(text);
      return true;
    }
    return false;
  },

  formatFileSize: (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  },

  getFileExtension: (filename: string): string => {
    return filename.slice((filename.lastIndexOf('.') - 1 >>> 0) + 2);
  },

  getLanguageFromExtension: (extension: string): string => {
    const languageMap: Record<string, string> = {
      'ts': 'typescript',
      'tsx': 'typescript',
      'js': 'javascript',
      'jsx': 'javascript',
      'py': 'python',
      'json': 'json',
      'html': 'html',
      'css': 'css',
      'scss': 'scss',
      'md': 'markdown',
      'yml': 'yaml',
      'yaml': 'yaml',
      'sh': 'shell',
      'bash': 'shell',
      'rs': 'rust',
      'toml': 'toml',
      'env': 'shell',
      'gitignore': 'gitignore',
    };
    return languageMap[extension.toLowerCase()] || 'plaintext';
  },
};

export const pathUtils = {
  joinPaths: (...paths: string[]): string => {
    return paths
      .map(path => path.replace(/^\/+|\/+$/g, ''))
      .filter(Boolean)
      .join('/');
  },

  getDirectoryName: (path: string): string => {
    return path.split('/').pop() || '';
  },

  getParentDirectory: (path: string): string => {
    const parts = path.split('/');
    parts.pop();
    return parts.join('/');
  },

  isValidProjectName: (name: string): boolean => {
    // Valid project names: alphanumeric, hyphens, underscores, no spaces
    return /^[a-zA-Z0-9_-]+$/.test(name) && name.length > 0;
  },

  sanitizeProjectName: (name: string): string => {
    return name.replace(/[^a-zA-Z0-9_-]/g, '-').replace(/-+/g, '-').toLowerCase();
  },
};