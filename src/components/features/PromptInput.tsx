import React, { useState } from 'react';
import { Zap, Loader, AlertCircle } from 'lucide-react';
import { useAppStore } from '../../stores/appStore';
import { tauriService } from '../../services/tauriService';
import { FrameworkSelector } from '../features/FrameworkSelector';

export const PromptInput: React.FC = () => {
  const { 
    isGenerating, 
    setGenerating, 
    setCurrentScaffold, 
    setGenerationError,
    settings 
  } = useAppStore();
  
  const [prompt, setPrompt] = useState('');
  const [selectedFramework, setSelectedFramework] = useState<string | undefined>();

  const handleSubmit = async () => {
    if (!prompt.trim()) return;
    
    if (!settings.apiKey) {
      alert('Please configure your OpenAI API key in Settings first.');
      return;
    }

    setGenerating(true);
    setGenerationError(undefined);
    
    try {
      const result = await tauriService.generateScaffold(
        { 
          prompt: prompt.trim(), 
          framework: selectedFramework 
        },
        settings.apiKey
      );
      
      setCurrentScaffold(result);
    } catch (error) {
      console.error('Generation failed:', error);
      setGenerationError(error instanceof Error ? error.message : 'Generation failed');
    } finally {
      setGenerating(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
      handleSubmit();
    }
  };

  return (
    <div className="space-y-6">
      <div className="bg-gray-900 border border-gray-700 rounded-lg p-6">
        <h2 className="text-lg font-semibold text-white mb-4">
          Describe Your Project
        </h2>
        
        <textarea
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder="e.g., Create a React todo app with Tailwind CSS and local storage, or a Flask REST API for managing books with CRUD endpoints"
          className="w-full h-32 px-4 py-3 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
          disabled={isGenerating}
        />
        
        <div className="flex items-center justify-between mt-4">
          <div className="text-sm text-gray-400">
            {prompt.length}/1000 characters • Press Cmd/Ctrl + Enter to generate
          </div>
          
          <button
            onClick={handleSubmit}
            disabled={!prompt.trim() || isGenerating || !settings.apiKey}
            className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white font-medium py-2 px-6 rounded-lg transition-colors flex items-center space-x-2"
          >
            {isGenerating ? (
              <>
                <Loader className="w-4 h-4 animate-spin" />
                <span>Generating...</span>
              </>
            ) : (
              <>
                <Zap className="w-4 h-4" />
                <span>Generate Project</span>
              </>
            )}
          </button>
        </div>
        
        {!settings.apiKey && (
          <div className="mt-4 flex items-center space-x-2 text-yellow-400 bg-yellow-900/20 border border-yellow-800 rounded-lg p-3">
            <AlertCircle className="w-4 h-4 flex-shrink-0" />
            <span className="text-sm">
              Please configure your OpenAI API key in Settings to generate projects.
            </span>
          </div>
        )}
      </div>

      <FrameworkSelector 
        selectedFramework={selectedFramework}
        onFrameworkSelect={setSelectedFramework}
      />
    </div>
  );
};