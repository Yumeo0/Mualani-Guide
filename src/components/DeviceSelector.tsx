import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/core';
import { DeviceInfo } from "../types";

function DeviceSelector() {
  const [devices, setDevices] = useState<DeviceInfo[]>([]);
  const [selectedDevice, setSelectedDevice] = useState<number>(0);

  useEffect(() => {
    async function fetchDevices() {
      try {
        const devicesList: DeviceInfo[] = await invoke("list_devices");
        setDevices(devicesList);
      } catch (error) {
        console.error("Failed to fetch devices:", error);
      }
    }

    fetchDevices();
  }, []);

  const handleSelect = async (index: number) => {
    setSelectedDevice(index);
    try {
        console.debug("set_device", index);
      await invoke("set_device", { index });
    } catch (error) {
      console.error("Failed to set device:", error);
    }
  };

  return (
    <div>
      <label htmlFor="device-dropdown">Select Device:</label>
      <select
        id="device-dropdown"
        value={selectedDevice || ""}
        onChange={(e) => handleSelect(Number(e.target.value))}
      >
        {devices.map((device) => (
          <option key={device.index} value={device.index}>
            {
                device.description ? `${device.description} (${device.name})` : device.name
            }   
          </option>
        ))}
      </select>
    </div>
  );
}

export default DeviceSelector;