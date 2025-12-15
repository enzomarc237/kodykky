import React, { useState, useEffect } from 'react';
import { History, ExternalLink, Trash2, Clock, Search } from 'lucide-react';
import { useAppStore } from '../stores/appStore';
import { tauriService } from '../services/tauriService';
import { ProjectHistory } from '../types';
import { formatDistanceToNow } from 'date-fns';

export const HistoryPage: React.FC = () => {
  const { projectHistory, removeFromHistory } = useAppStore();
  const [searchQuery, setSearchQuery] = useState('');
  const [filteredHistory, setFilteredHistory] = useState<ProjectHistory[]>([]);

  useEffect(() => {
    // Load project history from storage
    const loadHistory = async () => {
      try {
        await tauriService.getProjectHistory();
        // Note: In a real implementation, we would sync this with the store
      } catch (error) {
        console.error('Failed to load project history:', error);
      }
    };
    loadHistory();
  }, []);

  useEffect(() => {
    const filtered = projectHistory.filter((project) =>
      project.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      project.prompt.toLowerCase().includes(searchQuery.toLowerCase()) ||
      project.framework.toLowerCase().includes(searchQuery.toLowerCase())
    );
    setFilteredHistory(filtered);
  }, [projectHistory, searchQuery]);

  const handleDelete = async (id: string) => {
    try {
      await tauriService.deleteFromHistory(id);
      removeFromHistory(id);
    } catch (error) {
      console.error('Failed to delete from history:', error);
    }
  };

  const handleOpenInFinder = async (path: string) => {
    try {
      await tauriService.openInFinder(path);
    } catch (error) {
      console.error('Failed to open in Finder:', error);
    }
  };

  const formatDate = (dateString: string) => {
    try {
      return formatDistanceToNow(new Date(dateString), { addSuffix: true });
    } catch {
      return 'Unknown time';
    }
  };

  return (
    <div className="max-w-6xl mx-auto space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <History className="w-8 h-8 text-blue-400" />
          <h1 className="text-3xl font-bold text-white">Project History</h1>
        </div>
      </div>

      {/* Search */}
      <div className="bg-gray-900 border border-gray-700 rounded-lg p-4">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-5 h-5 text-gray-400" />
          <input
            type="text"
            placeholder="Search projects..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-10 pr-4 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
        </div>
      </div>

      {/* Project Grid */}
      {filteredHistory.length === 0 ? (
        <div className="bg-gray-900 border border-gray-700 rounded-lg p-12 text-center">
          <History className="w-16 h-16 text-gray-600 mx-auto mb-4" />
          {searchQuery ? (
            <div>
              <h3 className="text-lg font-semibold text-white mb-2">No matching projects</h3>
              <p className="text-gray-400">
                Try adjusting your search terms or create a new project.
              </p>
            </div>
          ) : (
            <div>
              <h3 className="text-lg font-semibold text-white mb-2">No projects yet</h3>
              <p className="text-gray-400">
                Your generated projects will appear here for easy access.
              </p>
            </div>
          )}
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredHistory.map((project) => (
            <div
              key={project.id}
              className="bg-gray-900 border border-gray-700 rounded-lg p-6 hover:bg-gray-800 transition-colors"
            >
              {/* Project Header */}
              <div className="flex items-start justify-between mb-4">
                <div className="flex-1 min-w-0">
                  <h3 className="text-lg font-semibold text-white truncate mb-1">
                    {project.name}
                  </h3>
                  <div className="flex items-center space-x-2 text-sm text-gray-400">
                    <span className="bg-gray-700 px-2 py-1 rounded text-xs">
                      {project.framework}
                    </span>
                    <div className="flex items-center space-x-1">
                      <Clock className="w-3 h-3" />
                      <span>{formatDate(project.created_at)}</span>
                    </div>
                  </div>
                </div>
              </div>

              {/* Project Description */}
              <div className="mb-4">
                <p className="text-gray-300 text-sm line-clamp-3">
                  {project.prompt}
                </p>
              </div>

              {/* Actions */}
              <div className="flex items-center justify-between pt-4 border-t border-gray-700">
                <button
                  onClick={() => handleOpenInFinder(project.path)}
                  className="flex items-center space-x-2 text-blue-400 hover:text-blue-300 transition-colors"
                >
                  <ExternalLink className="w-4 h-4" />
                  <span className="text-sm">Open Location</span>
                </button>
                
                <button
                  onClick={() => handleDelete(project.id)}
                  className="p-2 text-gray-400 hover:text-red-400 transition-colors"
                  title="Remove from history"
                >
                  <Trash2 className="w-4 h-4" />
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};