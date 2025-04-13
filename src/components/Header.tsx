
import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { Shield, Settings } from 'lucide-react';
import { Button } from '@/components/ui/button';

const Header = () => {
  const location = useLocation();

  return (
    <header className="bg-gradient-to-r from-indigo-800 to-purple-900 p-4 text-white">
      <div className="container mx-auto flex items-center justify-between">
        <div className="flex items-center space-x-2">
          <Shield className="h-8 w-8" />
          <Link to="/">
            <h1 className="text-2xl font-bold">RustBlocker</h1>
          </Link>
        </div>
        <div className="flex items-center space-x-4">
          <div className="text-sm hidden md:block">
            <span className="opacity-75">A native DNS-based ad blocker</span>
          </div>
          <nav>
            <Link to="/settings">
              <Button variant="ghost" size="sm" className="text-white hover:bg-white/10">
                <Settings className="h-5 w-5 mr-2" />
                Settings
              </Button>
            </Link>
          </nav>
        </div>
      </div>
    </header>
  );
};

export default Header;
