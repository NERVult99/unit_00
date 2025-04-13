
import React from 'react';
import Header from '@/components/Header';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Label } from '@/components/ui/label';
import { Settings as SettingsIcon } from 'lucide-react';

const Settings = () => {
  return (
    <div className="min-h-screen bg-gray-50">
      <Header />
      <main className="container mx-auto p-4">
        <div className="mb-6 flex items-center">
          <SettingsIcon className="mr-2 h-6 w-6" />
          <h2 className="text-2xl font-bold">Settings</h2>
        </div>

        <div className="grid gap-6">
          <Card>
            <CardHeader>
              <CardTitle>DNS Settings</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <Label htmlFor="dns-port">DNS Port</Label>
                <Input id="dns-port" type="number" defaultValue={5353} className="max-w-xs" />
                <p className="text-sm text-gray-500 mt-1">
                  Port for the DNS server (default: 5353)
                </p>
              </div>

              <div>
                <Label htmlFor="upstream-dns">Upstream DNS Server</Label>
                <Select defaultValue="google">
                  <SelectTrigger className="max-w-xs">
                    <SelectValue placeholder="Select a DNS provider" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="google">Google (8.8.8.8)</SelectItem>
                    <SelectItem value="cloudflare">Cloudflare (1.1.1.1)</SelectItem>
                    <SelectItem value="quad9">Quad9 (9.9.9.9)</SelectItem>
                    <SelectItem value="opendns">OpenDNS (208.67.222.222)</SelectItem>
                  </SelectContent>
                </Select>
                <p className="text-sm text-gray-500 mt-1">
                  DNS server to forward queries to
                </p>
              </div>

              <div className="pt-4">
                <Button>Save DNS Settings</Button>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Blocklist Management</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <Label htmlFor="custom-domain">Add Domain to Blocklist</Label>
                <div className="flex gap-2 max-w-md">
                  <Input id="custom-domain" placeholder="example.com" />
                  <Button>Add</Button>
                </div>
              </div>

              <div className="pt-2">
                <Button variant="outline">Update Blocklists</Button>
                <p className="text-sm text-gray-500 mt-1">
                  Download and update blocklists from online sources
                </p>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Application Settings</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center space-x-2">
                <Label htmlFor="startup" className="cursor-pointer">Start on system boot</Label>
                <Input id="startup" type="checkbox" className="w-4 h-4" />
              </div>

              <div className="pt-4">
                <Button variant="destructive">Reset All Settings</Button>
              </div>
            </CardContent>
          </Card>
        </div>
      </main>
    </div>
  );
};

export default Settings;
