import React, { useState } from 'react';
import { Code, Download, Folder, FileText, Loader } from 'lucide-react';
import { useAppStore } from '../../stores/appStore';
import { tauriService, pathUtils } from '../../services/tauriService';
import { ProjectHistory } from '../../types';
import { CodePreview } from './CodePreview';

export const ProjectPreview: React.FC = () => {
  const { 
    currentScaffold, 
    selectedFile, 
    setSelectedFile,
    settings,
    addToHistory,
    isGenerating 
  } = useAppStore();
  
  const [showExportDialog, setShowExportDialog] = useState(false);
  const [exportPath, setExportPath] = useState(settings.defaultProjectLocation || '');
  const [isExporting, setIsExporting] = useState(false);
  const [exportError, setExportError] = useState<string | undefined>();

  if (isGenerating) {
    return (
      <div className="bg-gray-900 border border-gray-700 rounded-lg p-12 text-center">
        <Loader className="w-12 h-12 text-blue-500 mx-auto mb-4 animate-spin" />
        <h3 className="text-lg font-semibold text-white mb-2">
          Generating Your Project
        </h3>
        <p className="text-gray-400">
          AI is analyzing your request and creating a custom project structure...
        </p>
      </div>
    );
  }

  if (!currentScaffold) {
    return (
      <div className="bg-gray-900 border border-gray-700 rounded-lg p-12 text-center">
                <Folder className="w-16 h-16 text-gray-600 mx-auto mb-4" />
                <h3 className="text-lg font-semibold text-white mb-2">
                  No Project Generated Yet
                </h3>
                <p className="text-gray-400">
                  Enter a prompt above to see your generated project structure and files.
                </p>
              </div>
    );
  }

  const handleExport = async () => {
    if (!exportPath.trim()) return;
    
    setIsExporting(true);
    setExportError(undefined);
    
    try {
      // Create project structure
      const projectStructure = {
        name: currentScaffold.project_name,
        framework: currentScaffold.framework,
        files: currentScaffold.files,
      };
      
      await tauriService.createProject(exportPath, projectStructure);
      
      // Add to history
      const project: ProjectHistory = {
        id: `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
        name: currentScaffold.project_name,
        prompt: 'Generated project', // We would store the actual prompt in a real implementation
        framework: currentScaffold.framework,
        path: pathUtils.joinPaths(exportPath, currentScaffold.project_name),
        created_at: new Date().toISOString(),
      };
      
      addToHistory(project);
      setShowExportDialog(false);
      alert('Project exported successfully!');
      
    } catch (error) {
      setExportError(error instanceof Error ? error.message : 'Export failed');
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <div className="space-y-6">
      {/* Project Overview */}
      <div className="bg-gray-900 border border-gray-700 rounded-lg p-6">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h2 className="text-2xl font-bold text-white">
              {currentScaffold.project_name}
            </h2>
            <p className="text-gray-400">
              {currentScaffold.framework} • {Object.keys(currentScaffold.files).length} files
            </p>
          </div>
          
          <div className="flex items-center space-x-2">
            <button
              onClick={() => setShowExportDialog(true)}
              className="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition-colors flex items-center space-x-2"
            >
              <Download className="w-4 h-4" />
              <span>Export Project</span>
            </button>
          </div>
        </div>

        {/* Dependencies */}
        {Object.keys(currentScaffold.dependencies).length > 0 && (
          <div>
            <h3 className="text-sm font-medium text-gray-300 mb-2">Dependencies</h3>
            <div className="flex flex-wrap gap-2">
              {Object.entries(currentScaffold.dependencies).map(([name, version]) => (
                <span
                  key={name}
                  className="bg-gray-800 text-gray-300 px-2 py-1 rounded text-sm"
                >
                  {name}@{version}
                </span>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* File Tree and Code Preview */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* File Tree */}
        <div className="bg-gray-900 border border-gray-700 rounded-lg p-6">
          <h3 className="text-lg font-semibold text-white mb-4 flex items-center space-x-2">
            <Folder className="w-5 h-5" />
            <span>Project Structure</span>
          </h3>
          
          <div className="space-y-1">
            {Object.keys(currentScaffold.files).map((filePath) => {
              const isSelected = selectedFile === filePath;
              const isDirectory = filePath.includes('/');
              
              return (
                <button
                  key={filePath}
                  onClick={() => setSelectedFile(filePath)}
                  className={`w-full text-left px-3 py-2 rounded transition-colors flex items-center space-x-2 ${
                    isSelected 
                      ? 'bg-blue-600 text-white' 
                      : 'text-gray-300 hover:bg-gray-800'
                  }`}
                >
                  {isDirectory ? (
                    <Folder className="w-4 h-4" />
                  ) : (
                    <FileText className="w-4 h-4" />
                  )}
                  <span className="text-sm truncate">{filePath}</span>
                </button>
              );
            })}
          </div>
        </div>

        {/* Code Preview */}
        <div className="bg-gray-900 border border-gray-700 rounded-lg p-6">
          <h3 className="text-lg font-semibold text-white mb-4 flex items-center space-x-2">
            <Code className="w-5 h-5" />
            <span>Code Preview</span>
          </h3>
          
          {selectedFile ? (
            <CodePreview 
              filePath={selectedFile}
              content={currentScaffold.files[selectedFile]}
            />
          ) : (
            <div className="flex items-center justify-center h-64 text-gray-400">
              <div className="text-center">
                <FileText className="w-12 h-12 mx-auto mb-3 opacity-50" />
                <p>Select a file to preview its content</p>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Export Dialog */}
      {showExportDialog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-gray-900 border border-gray-700 rounded-lg p-6 w-full max-w-md">
            <h3 className="text-lg font-semibold text-white mb-4">
              Export Project
            </h3>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">
                  Export Location
                </label>
                <input
                  type="text"
                  value={exportPath}
                  onChange={(e) => setExportPath(e.target.value)}
                  placeholder="/path/to/projects"
                  className="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
              </div>
              
              {exportError && (
                <div className="text-red-400 text-sm">
                  {exportError}
                </div>
              )}
              
              <div className="flex items-center justify-end space-x-2">
                <button
                  onClick={() => setShowExportDialog(false)}
                  disabled={isExporting}
                  className="px-4 py-2 text-gray-400 hover:text-white transition-colors"
                >
                  Cancel
                </button>
                <button
                  onClick={handleExport}
                  disabled={!exportPath.trim() || isExporting}
                  className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white font-medium py-2 px-4 rounded-lg transition-colors flex items-center space-x-2"
                >
                  {isExporting ? (
                    <>
                      <Loader className="w-4 h-4 animate-spin" />
                      <span>Exporting...</span>
                    </>
                  ) : (
                    <>
                      <Download className="w-4 h-4" />
                      <span>Export</span>
                    </>
                  )}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};