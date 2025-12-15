import React, { useState } from 'react';
import { Settings as SettingsIcon, Key, Folder, Code, Palette, Save } from 'lucide-react';
import { useAppStore } from '../stores/appStore';
import { tauriService } from '../services/tauriService';

export const SettingsPage: React.FC = () => {
  const { settings, setSettings } = useAppStore();
  const [apiKey, setApiKey] = useState(settings.apiKey || '');
  const [isValidating, setIsValidating] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [validationResult, setValidationResult] = useState<'valid' | 'invalid' | undefined>();

  const handleSaveApiKey = async () => {
    if (!apiKey.trim()) return;
    
    setIsSaving(true);
    try {
      await tauriService.saveApiKey(apiKey.trim());
      setSettings({ apiKey: apiKey.trim() });
      alert('API key saved successfully');
    } catch (error) {
      alert(`Failed to save API key: ${error}`);
    } finally {
      setIsSaving(false);
    }
  };

  const handleValidateApiKey = async () => {
    if (!apiKey.trim()) return;
    
    setIsValidating(true);
    try {
      const isValid = await tauriService.validateApiKey(apiKey.trim());
      setValidationResult(isValid ? 'valid' : 'invalid');
      if (isValid) {
        setSettings({ apiKey: apiKey.trim() });
      }
    } catch (error) {
      setValidationResult('invalid');
    } finally {
      setIsValidating(false);
    }
  };

  return (
    <div className="max-w-4xl mx-auto space-y-8">
      {/* Header */}
      <div className="flex items-center space-x-3">
        <SettingsIcon className="w-8 h-8 text-blue-400" />
        <h1 className="text-3xl font-bold text-white">Settings</h1>
      </div>

      {/* OpenAI API Key Section */}
      <div className="bg-gray-900 border border-gray-700 rounded-lg p-6">
        <div className="flex items-center space-x-2 mb-4">
          <Key className="w-5 h-5 text-blue-400" />
          <h2 className="text-xl font-semibold text-white">OpenAI API Key</h2>
        </div>
        
        <p className="text-gray-400 mb-4">
          Your API key is stored securely in your system's keychain and is never shared.
        </p>

        <div className="space-y-4">
          <div>
            <label htmlFor="api-key" className="block text-sm font-medium text-gray-300 mb-2">
              API Key
            </label>
            <div className="flex space-x-2">
              <input
                id="api-key"
                type="password"
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                placeholder="sk-..."
                className="flex-1 px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              />
              <button
                onClick={handleValidateApiKey}
                disabled={!apiKey.trim() || isValidating}
                className="px-4 py-2 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 text-white font-medium rounded-lg transition-colors flex items-center space-x-2"
              >
                {isValidating ? (
                  <>
                    <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                    <span>Validating...</span>
                  </>
                ) : (
                  <span>Validate</span>
                )}
              </button>
              <button
                onClick={handleSaveApiKey}
                disabled={!apiKey.trim() || isSaving}
                className="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white font-medium rounded-lg transition-colors flex items-center space-x-2"
              >
                {isSaving ? (
                  <>
                    <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                    <span>Saving...</span>
                  </>
                ) : (
                  <>
                    <Save className="w-4 h-4" />
                    <span>Save</span>
                  </>
                )}
              </button>
            </div>
            
            {validationResult === 'valid' && (
              <div className="mt-2 text-sm text-green-400 flex items-center space-x-1">
                <div className="w-2 h-2 bg-green-400 rounded-full"></div>
                <span>Valid API key</span>
              </div>
            )}
            
            {validationResult === 'invalid' && (
              <div className="mt-2 text-sm text-red-400 flex items-center space-x-1">
                <div className="w-2 h-2 bg-red-400 rounded-full"></div>
                <span>Invalid API key</span>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Editor Settings */}
      <div className="bg-gray-900 border border-gray-700 rounded-lg p-6">
        <div className="flex items-center space-x-2 mb-4">
          <Code className="w-5 h-5 text-blue-400" />
          <h2 className="text-xl font-semibold text-white">Editor Preferences</h2>
        </div>
        
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Preferred Editor
            </label>
            <select
              value={settings.preferredEditor}
              onChange={(e) => setSettings({ preferredEditor: e.target.value as any })}
              className="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              <option value="vscode">Visual Studio Code</option>
              <option value="vscode-insiders">Visual Studio Code Insiders</option>
              <option value="default">System Default</option>
            </select>
          </div>
        </div>
      </div>

      {/* Default Project Location */}
      <div className="bg-gray-900 border border-gray-700 rounded-lg p-6">
        <div className="flex items-center space-x-2 mb-4">
          <Folder className="w-5 h-5 text-blue-400" />
          <h2 className="text-xl font-semibold text-white">Project Location</h2>
        </div>
        
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Default Project Directory
            </label>
            <input
              type="text"
              value={settings.defaultProjectLocation || ''}
              onChange={(e) => setSettings({ defaultProjectLocation: e.target.value })}
              placeholder="/Users/username/Projects"
              className="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>
        </div>
      </div>

      {/* History Settings */}
      <div className="bg-gray-900 border border-gray-700 rounded-lg p-6">
        <div className="flex items-center space-x-2 mb-4">
          <Palette className="w-5 h-5 text-blue-400" />
          <h2 className="text-xl font-semibold text-white">History & Preferences</h2>
        </div>
        
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <label className="text-sm font-medium text-gray-300">
                Auto-save project history
              </label>
              <p className="text-xs text-gray-400">
                Automatically save projects to history for quick access
              </p>
            </div>
            <input
              type="checkbox"
              checked={settings.autoSaveHistory}
              onChange={(e) => setSettings({ autoSaveHistory: e.target.checked })}
              className="w-4 h-4 text-blue-600 bg-gray-800 border-gray-600 rounded focus:ring-blue-500 focus:ring-2"
            />
          </div>
          
          <div className="flex items-center justify-between">
            <div>
              <label className="text-sm font-medium text-gray-300">
                Confirm before overwriting files
              </label>
              <p className="text-xs text-gray-400">
                Show confirmation dialog when overwriting existing files
              </p>
            </div>
            <input
              type="checkbox"
              checked={settings.confirmBeforeOverwrite}
              onChange={(e) => setSettings({ confirmBeforeOverwrite: e.target.checked })}
              className="w-4 h-4 text-blue-600 bg-gray-800 border-gray-600 rounded focus:ring-blue-500 focus:ring-2"
            />
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Max history items
            </label>
            <input
              type="number"
              min="10"
              max="100"
              value={settings.maxHistoryItems}
              onChange={(e) => setSettings({ maxHistoryItems: parseInt(e.target.value) })}
              className="w-24 px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>
        </div>
      </div>
    </div>
  );
};