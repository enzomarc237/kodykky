import React, { useState } from 'react';
import { Copy, Check } from 'lucide-react';
import { fileUtils } from '../../services/tauriService';

interface CodePreviewProps {
  filePath: string;
  content: string;
}

export const CodePreview: React.FC<CodePreviewProps> = ({ filePath, content }) => {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    const success = await fileUtils.copyToClipboard(content);
    if (success) {
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const extension = fileUtils.getFileExtension(filePath);
  const language = fileUtils.getLanguageFromExtension(extension);

  return (
    <div className="flex flex-col h-full">
      {/* File Header */}
      <div className="flex items-center justify-between mb-3 pb-3 border-b border-gray-700">
        <div className="flex items-center space-x-2">
          <span className="text-sm font-medium text-gray-300">
            {filePath}
          </span>
          <span className="text-xs text-gray-500 bg-gray-800 px-2 py-1 rounded">
            {language}
          </span>
        </div>
        
        <button
          onClick={handleCopy}
          className="flex items-center space-x-1 text-gray-400 hover:text-white transition-colors text-sm"
          title="Copy to clipboard"
        >
          {copied ? (
            <>
              <Check className="w-4 h-4 text-green-400" />
              <span className="text-green-400">Copied!</span>
            </>
          ) : (
            <>
              <Copy className="w-4 h-4" />
              <span>Copy</span>
            </>
          )}
        </button>
      </div>

      {/* Code Content */}
      <div className="flex-1 overflow-auto">
        <pre className="text-sm text-gray-300 bg-gray-950 p-4 rounded border border-gray-700 overflow-auto">
          <code className={`language-${language}`}>
            {content}
          </code>
        </pre>
      </div>
    </div>
  );
};