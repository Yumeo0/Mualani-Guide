import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';

export default function PacketSnifferControl() {
  const [isCapturing, setIsCapturing] = useState(false);
  const [status, setStatus] = useState('');
  const [error, setError] = useState<string | null>(null);

  const handleStartCapture = async () => {
    try {
      setError(null);
      const response = await invoke('start_packet_capture');
      setStatus(response as string);
      setIsCapturing(true);
    } catch (err) {
      setError(err as string);
      setIsCapturing(false);
    }
  };

  const handleStopCapture = async () => {
    try {
      setError(null);
      const response = await invoke('stop_packet_capture');
      setStatus(response as string);
      setIsCapturing(false);
    } catch (err) {
      setError(err as string);
    }
  };

  return (
    <div className="p-4">
      <div className="flex flex-col gap-4">
        <div className="flex gap-4">
          <button
            onClick={handleStartCapture}
            disabled={isCapturing}
            className={`px-4 py-2 rounded transition-colors ${
              isCapturing 
                ? 'bg-gray-300 cursor-not-allowed' 
                : 'bg-green-500 hover:bg-green-600'
            } text-white font-medium`}
          >
            Start Packet Capture
          </button>
          <button
            onClick={handleStopCapture}
            disabled={!isCapturing}
            className={`px-4 py-2 rounded transition-colors ${
              !isCapturing 
                ? 'bg-gray-300 cursor-not-allowed' 
                : 'bg-red-500 hover:bg-red-600'
            } text-white font-medium`}
          >
            Stop Packet Capture
          </button>
        </div>
        
        {status && (
          <div className="p-3 bg-gray-100 rounded">
            <p className="text-gray-700">{status}</p>
          </div>
        )}
        
        {error && (
          <div className="p-3 bg-red-100 rounded">
            <p className="text-red-700">Error: {error}</p>
          </div>
        )}
      </div>
    </div>
  );
}