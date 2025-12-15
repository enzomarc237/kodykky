import React from 'react';
import { useAppStore } from '../../stores/appStore';
import { Clock, Trash2, ExternalLink } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';

export const Sidebar: React.FC = () => {
  const { projectHistory, removeFromHistory } = useAppStore();

  const formatDate = (dateString: string) => {
    try {
      return formatDistanceToNow(new Date(dateString), { addSuffix: true });
    } catch {
      return 'Unknown time';
    }
  };

  return (
    <aside className="w-80 bg-gray-900 border-r border-gray-700 p-4">
      <div className="flex items-center space-x-2 mb-4">
        <Clock className="w-5 h-5 text-gray-400" />
        <h2 className="text-lg font-semibold text-white">Project History</h2>
      </div>
      
      <div className="space-y-2">
        {projectHistory.length === 0 ? (
          <div className="text-center py-8">
            <Clock className="w-12 h-12 text-gray-600 mx-auto mb-3" />
            <p className="text-gray-400 text-sm">No projects created yet</p>
          </div>
        ) : (
          projectHistory.map((project) => (
            <div
              key={project.id}
              className="bg-gray-800 rounded-lg p-3 hover:bg-gray-750 transition-colors"
            >
              <div className="flex items-start justify-between">
                <div className="flex-1 min-w-0">
                  <h3 className="text-sm font-medium text-white truncate">
                    {project.name}
                  </h3>
                  <p className="text-xs text-gray-400 mt-1">
                    {project.framework} • {formatDate(project.created_at)}
                  </p>
                  <p className="text-xs text-gray-500 mt-1 line-clamp-2">
                    {project.prompt}
                  </p>
                </div>
                
                <div className="flex items-center space-x-1 ml-2">
                  <button
                    onClick={() => window.open(`file://${project.path}`, '_blank')}
                    className="p-1 text-gray-400 hover:text-white transition-colors"
                    title="Open in Finder"
                  >
                    <ExternalLink className="w-3 h-3" />
                  </button>
                  <button
                    onClick={() => removeFromHistory(project.id)}
                    className="p-1 text-gray-400 hover:text-red-400 transition-colors"
                    title="Remove from history"
                  >
                    <Trash2 className="w-3 h-3" />
                  </button>
                </div>
              </div>
            </div>
          ))
        )}
      </div>
    </aside>
  );
};