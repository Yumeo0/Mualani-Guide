<template>
  <div>
    <label for="device-dropdown">Select Device:</label>
    <select id="device-dropdown" :value="selectedDevice || 0"
      @change="(e: Event) => handleSelect(Number((e.target as HTMLSelectElement).value))">
      <option v-for="device in devices" :key="device.index" :value="device.index">
        {{ device.description ? `${device.description} (${device.name})` : device.name }}
      </option>
    </select>
  </div>
</template>

<script lang="ts" setup>
  import type { DeviceInfo } from '~/types/types';
  import { invoke } from '@tauri-apps/api/core';

  const devices = ref<DeviceInfo[]>([]);
  const selectedDevice = ref<number>(0);

  const res = await invoke<DeviceInfo[]>(TAURI_COMMANDS.list_devices);
  devices.value = res;

  async function handleSelect(index: number) {
    try {
      console.debug("set_device", index);
      selectedDevice.value = index;
      await invoke("set_device", { index });
    } catch (error) {
      console.error("Failed to set device:", error);
    }
  }
</script>

<style></style>