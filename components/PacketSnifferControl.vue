<template>
  <div class="p-4">
    <div class="flex flex-col gap-4">
      <div class="flex gap-4">
        <button @click="handleStartCapture" :disabled="isCapturing" :class="`px-4 py-2 rounded transition-colors
          ${isCapturing ? 'bg-gray-300 cursor-not-allowed' : 'bg-green-500 hover:bg-green-600'} text-white
          font-medium`">
          Start Packet Capture
        </button>
        <button @click="handleStopCapture" :disabled="!isCapturing"
          :class="`px-4 py-2 rounded transition-colors ${!isCapturing ? 'bg-gray-300 cursor-not-allowed' : 'bg-red-500 hover:bg-red-600'} text-white font-medium`">
          Stop Packet Capture
        </button>
      </div>

      <div v-if="status" class="p-3 bg-gray-100 rounded">
        <p class="text-gray-700">{{ status }}</p>
      </div>

      <div v-if="error" class="p-3 bg-red-100 rounded">
        <p class="text-red-700">Error: {{ error }}</p>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import { invoke } from '@tauri-apps/api/core';

  const isCapturing = ref(false);
  const status = ref('');
  const error = ref<string | null>(null);


  const handleStartCapture = async () => {
    try {
      error.value = null;
      const response = await invoke<string>(TAURI_COMMANDS.start_packet_capture);
      status.value = response;
      isCapturing.value = true;
    } catch (err) {
      error.value = err as string;
      isCapturing.value = false;
    }
  };

  const handleStopCapture = async () => {
    try {
      error.value = null;
      const response = await invoke<string>('stop_packet_capture');
      status.value = response;
      isCapturing.value = false;
    } catch (err) {
      error.value = err as string;
    }
  }
</script>

<style></style>