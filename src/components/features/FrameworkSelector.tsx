import React from 'react';
import { SUPPORTED_FRAMEWORKS } from '../../types';

interface FrameworkSelectorProps {
  selectedFramework?: string;
  onFrameworkSelect: (framework: string | undefined) => void;
}

export const FrameworkSelector: React.FC<FrameworkSelectorProps> = ({
  selectedFramework,
  onFrameworkSelect,
}) => {
  return (
    <div className="bg-gray-900 border border-gray-700 rounded-lg p-6">
      <h3 className="text-lg font-semibold text-white mb-4">
        Choose Framework (Optional)
      </h3>
      <p className="text-sm text-gray-400 mb-4">
        Select a specific framework or let AI detect the best option from your description.
      </p>
      
      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4">
        {/* Auto-detect option */}
        <button
          onClick={() => onFrameworkSelect(undefined)}
          className={`bg-gray-800 hover:bg-gray-700 border rounded-lg p-4 transition-colors text-center ${
            !selectedFramework 
              ? 'border-blue-500 bg-blue-900/20' 
              : 'border-gray-600'
          }`}
        >
          <div className="text-2xl mb-2">🤖</div>
          <div className="text-sm font-medium text-white">Auto-detect</div>
          <div className="text-xs text-gray-400 mt-1">Let AI choose</div>
        </button>

        {/* Framework options */}
        {SUPPORTED_FRAMEWORKS.map((framework) => (
          <button
            key={framework.id}
            onClick={() => onFrameworkSelect(framework.id)}
            className={`bg-gray-800 hover:bg-gray-700 border rounded-lg p-4 transition-colors text-center ${
              selectedFramework === framework.id 
                ? 'border-blue-500 bg-blue-900/20' 
                : 'border-gray-600'
            }`}
            style={{
              borderColor: selectedFramework === framework.id ? framework.color : undefined,
            }}
          >
            <div 
              className="text-2xl mb-2"
              style={{ filter: 'grayscale(0.3)' }}
            >
              {framework.icon}
            </div>
            <div className="text-sm font-medium text-white">{framework.name}</div>
            <div className="text-xs text-gray-400 mt-1 line-clamp-2">
              {framework.description}
            </div>
          </button>
        ))}
      </div>
    </div>
  );
};