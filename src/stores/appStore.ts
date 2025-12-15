import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { AppState, ScaffoldResponse, ProjectHistory, AppSettings } from '../types';

interface AppStore extends AppState {
  // Actions
  setGenerating: (isGenerating: boolean) => void;
  setCurrentScaffold: (scaffold: ScaffoldResponse | undefined) => void;
  setGenerationError: (error: string | undefined) => void;
  setSelectedFramework: (framework: string | undefined) => void;
  setPreviewMode: (mode: 'tree' | 'code') => void;
  setSelectedFile: (file: string | undefined) => void;
  setSettingsOpen: (isOpen: boolean) => void;
  setSettings: (settings: Partial<AppSettings>) => void;
  updateProjectHistory: (history: ProjectHistory[]) => void;
  addToHistory: (project: ProjectHistory) => void;
  removeFromHistory: (id: string) => void;
  reset: () => void;
}

const defaultSettings: AppSettings = {
  preferredEditor: 'vscode',
  theme: 'dark',
  autoSaveHistory: true,
  maxHistoryItems: 50,
  confirmBeforeOverwrite: true,
};

export const useAppStore = create<AppStore>()(
  persist(
    (set, get) => ({
      // Initial state
      isGenerating: false,
      projectHistory: [],
      settings: defaultSettings,
      previewMode: 'tree',
      isSettingsOpen: false,

      // Actions
      setGenerating: (isGenerating) => set({ isGenerating }),
      
      setCurrentScaffold: (currentScaffold) => {
        set({ currentScaffold, generationError: undefined });
      },
      
      setGenerationError: (generationError) => {
        set({ generationError });
      },
      
      setSelectedFramework: (selectedFramework) => {
        set({ selectedFramework });
      },
      
      setPreviewMode: (previewMode) => {
        set({ previewMode });
      },
      
      setSelectedFile: (selectedFile) => {
        set({ selectedFile });
      },
      
      setSettingsOpen: (isSettingsOpen) => {
        set({ isSettingsOpen });
      },
      
      setSettings: (newSettings) => {
        const currentSettings = get().settings;
        set({ settings: { ...currentSettings, ...newSettings } });
      },
      
      updateProjectHistory: (projectHistory) => {
        set({ projectHistory });
      },
      
      addToHistory: (project) => {
        const currentHistory = get().projectHistory;
        const updatedHistory = [project, ...currentHistory];
        const maxItems = get().settings.maxHistoryItems;
        
        if (updatedHistory.length > maxItems) {
          updatedHistory.splice(maxItems);
        }
        
        set({ projectHistory: updatedHistory });
      },
      
      removeFromHistory: (id) => {
        const currentHistory = get().projectHistory;
        const updatedHistory = currentHistory.filter(p => p.id !== id);
        set({ projectHistory: updatedHistory });
      },
      
      reset: () => {
        set({
          isGenerating: false,
          currentScaffold: undefined,
          generationError: undefined,
          selectedFramework: undefined,
          previewMode: 'tree',
          selectedFile: undefined,
          isSettingsOpen: false,
        });
      },
    }),
    {
      name: 'scaffold-ai-store',
      partialize: (state) => ({
        projectHistory: state.projectHistory,
        settings: state.settings,
      }),
    }
  )
);