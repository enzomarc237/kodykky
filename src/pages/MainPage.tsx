import React from 'react';
import { Zap } from 'lucide-react';
import { PromptInput } from '../components/features/PromptInput';
import { ProjectPreview } from '../components/features/ProjectPreview';

export const MainPage: React.FC = () => {
  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="text-center">
        <div className="flex items-center justify-center space-x-3 mb-4">
          <Zap className="w-10 h-10 text-blue-400" />
          <h1 className="text-4xl font-bold text-white">
            Create Projects with AI
          </h1>
        </div>
        <p className="text-xl text-gray-400 max-w-2xl mx-auto">
          Describe your project idea in natural language, and let AI generate 
          a complete, working codebase for you.
        </p>
      </div>

      {/* Prompt Input */}
      <div className="max-w-4xl mx-auto">
        <PromptInput />
      </div>

      {/* Project Preview */}
      <div className="max-w-6xl mx-auto">
        <ProjectPreview />
      </div>
    </div>
  );
};