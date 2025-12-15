import React from 'react';
import { Settings, History, Zap } from 'lucide-react';
import { useNavigate, useLocation } from 'react-router-dom';

export const Navbar: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();

  const navItems = [
    { path: '/', icon: Zap, label: 'Generate' },
    { path: '/history', icon: History, label: 'History' },
    { path: '/settings', icon: Settings, label: 'Settings' },
  ];

  return (
    <nav className="bg-gray-900 border-b border-gray-700 px-6 py-3">
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-2">
          <Zap className="w-6 h-6 text-blue-400" />
          <h1 className="text-xl font-bold text-white">Scaffold AI</h1>
        </div>
        
        <div className="flex items-center space-x-1">
          {navItems.map(({ path, icon: Icon, label }) => (
            <button
              key={path}
              onClick={() => navigate(path)}
              className={`flex items-center space-x-2 px-4 py-2 rounded-lg transition-colors ${
                location.pathname === path
                  ? 'bg-blue-600 text-white'
                  : 'text-gray-300 hover:bg-gray-800 hover:text-white'
              }`}
            >
              <Icon className="w-4 h-4" />
              <span className="text-sm font-medium">{label}</span>
            </button>
          ))}
        </div>
      </div>
    </nav>
  );
};