
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Shield, ShieldOff, FileText, AlertTriangle } from 'lucide-react';
import { useToast } from '@/hooks/use-toast';
import { Progress } from '@/components/ui/progress';
import { Switch } from '@/components/ui/switch';
import { open } from '@tauri-apps/api/dialog';

interface DnsStats {
  total_queries: number;
  blocked_queries: number;
  blocklist_size: number;
}

interface DnsQuery {
  domain: string;
  timestamp: string;
  blocked: boolean;
}

const Dashboard = () => {
  const [isRunning, setIsRunning] = useState(false);
  const [stats, setStats] = useState<DnsStats>({ total_queries: 0, blocked_queries: 0, blocklist_size: 0 });
  const [recentQueries, setRecentQueries] = useState<DnsQuery[]>([]);
  const [isLoadingBlocklist, setIsLoadingBlocklist] = useState(false);
  const { toast } = useToast();

  const fetchStatus = async () => {
    try {
      const running = await invoke<boolean>('get_dns_status');
      setIsRunning(running);
    } catch (error) {
      console.error('Error fetching DNS status:', error);
    }
  };

  const fetchStats = async () => {
    try {
      const stats = await invoke<DnsStats>('get_stats');
      setStats(stats);
    } catch (error) {
      console.error('Error fetching stats:', error);
    }
  };

  const fetchRecentQueries = async () => {
    try {
      const queries = await invoke<DnsQuery[]>('get_recent_queries');
      setRecentQueries(queries);
    } catch (error) {
      console.error('Error fetching recent queries:', error);
    }
  };

  const toggleDnsServer = async () => {
    try {
      if (isRunning) {
        await invoke('stop_dns_server');
        toast({
          title: 'DNS Blocker Stopped',
          description: 'DNS blocking has been disabled.',
        });
      } else {
        await invoke('start_dns_server');
        toast({
          title: 'DNS Blocker Started',
          description: 'DNS blocking is now active.',
        });
      }
      await fetchStatus();
    } catch (error) {
      console.error('Error toggling DNS server:', error);
      toast({
        title: 'Error',
        description: `Failed to ${isRunning ? 'stop' : 'start'} DNS server: ${error}`,
        variant: 'destructive',
      });
    }
  };

  const loadBlocklistFromFile = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Blocklist',
          extensions: ['txt']
        }]
      });
      
      if (selected && typeof selected === 'string') {
        setIsLoadingBlocklist(true);
        const count = await invoke<number>('load_blocklist_from_file', { filePath: selected });
        toast({
          title: 'Blocklist Loaded',
          description: `Successfully loaded ${count} domains to blocklist.`,
        });
        await fetchStats();
      }
    } catch (error) {
      console.error('Error loading blocklist:', error);
      toast({
        title: 'Error',
        description: `Failed to load blocklist: ${error}`,
        variant: 'destructive',
      });
    } finally {
      setIsLoadingBlocklist(false);
    }
  };

  useEffect(() => {
    fetchStatus();
    fetchStats();
    fetchRecentQueries();

    const interval = setInterval(() => {
      fetchStats();
      fetchRecentQueries();
    }, 5000);

    return () => clearInterval(interval);
  }, []);

  const blockPercentage = stats.total_queries > 0 
    ? Math.round((stats.blocked_queries / stats.total_queries) * 100)
    : 0;

  return (
    <div className="container mx-auto p-4">
      <div className="flex flex-col space-y-4">
        {/* Status Card */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center justify-between">
              <div className="flex items-center space-x-2">
                {isRunning ? (
                  <Shield className="h-5 w-5 text-green-500" />
                ) : (
                  <ShieldOff className="h-5 w-5 text-gray-500" />
                )}
                <span>DNS Blocker Status</span>
              </div>
              <Switch 
                checked={isRunning}
                onCheckedChange={toggleDnsServer}
              />
            </CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-center text-xl font-medium">
              {isRunning ? (
                <span className="text-green-500">Protecting your network</span>
              ) : (
                <span className="text-gray-500">Protection disabled</span>
              )}
            </p>
            <div className="mt-4">
              <Button onClick={loadBlocklistFromFile} variant="outline" disabled={isLoadingBlocklist} className="w-full">
                {isLoadingBlocklist ? 'Loading...' : 'Load Blocklist from File'}
              </Button>
            </div>
          </CardContent>
        </Card>

        {/* Stats Cards */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">Total DNS Queries</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-3xl font-bold">{stats.total_queries}</p>
            </CardContent>
          </Card>
          
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">Blocked Queries</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-3xl font-bold text-red-500">{stats.blocked_queries}</p>
              <Progress value={blockPercentage} className="mt-2" />
              <p className="text-sm text-gray-500 mt-1">{blockPercentage}% of total traffic</p>
            </CardContent>
          </Card>
          
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">Domains in Blocklist</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-3xl font-bold">{stats.blocklist_size}</p>
            </CardContent>
          </Card>
        </div>

        {/* Recent Queries */}
        <Card>
          <CardHeader>
            <CardTitle>Recent DNS Queries</CardTitle>
          </CardHeader>
          <CardContent>
            {recentQueries.length > 0 ? (
              <div className="overflow-x-auto">
                <table className="w-full text-sm">
                  <thead>
                    <tr>
                      <th className="text-left pb-2">Domain</th>
                      <th className="text-left pb-2">Time</th>
                      <th className="text-left pb-2">Status</th>
                    </tr>
                  </thead>
                  <tbody>
                    {recentQueries.slice(0, 10).map((query, index) => (
                      <tr key={index} className={index % 2 === 0 ? 'bg-gray-50' : ''}>
                        <td className="py-2">{query.domain}</td>
                        <td className="py-2">{new Date(query.timestamp).toLocaleTimeString()}</td>
                        <td className="py-2">
                          {query.blocked ? (
                            <span className="flex items-center text-red-500">
                              <AlertTriangle className="h-3 w-3 mr-1" /> Blocked
                            </span>
                          ) : (
                            <span className="text-green-500">Allowed</span>
                          )}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            ) : (
              <p className="text-center text-gray-500 py-4">No queries recorded yet</p>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
};

export default Dashboard;
