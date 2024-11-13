<script lang="ts" setup>
  import type { ManualTextMap, ReliquaryAffixData, ReliquaryData, ReliquaryMainPropData, TextMap } from './types/ExcelConfigDataTypes';
  import { listen, type Event } from '@tauri-apps/api/event';

  const resources = useResources();
  const inventory = useInventoryStore();

  await callOnce("loadResources", async () => {
    resources.value.artifactData = await readData<ReliquaryData[]>('resources/gi/data/ReliquaryExcelConfigData.json') ?? [];
    resources.value.artifactMstatData = await readData<ReliquaryMainPropData[]>('resources/gi/data/ReliquaryMainPropExcelConfigData.json') ?? [];
    resources.value.artifactSstatData = await readData<ReliquaryAffixData[]>('resources/gi/data/ReliquaryAffixExcelConfigData.json') ?? [];
    resources.value.manualTextMap = await readData<ManualTextMap[]>('resources/gi/data/ManualTextMapConfigData.json') ?? [];
    resources.value.textMap = await readData<TextMap>('resources/gi/data/TextMapEN.json') ?? null;
  })

  await callOnce("registerEvents", async () => {
    listen(EVENTS.store_notify, (event: Event<string>) => {
      const data = JSON.parse(event.payload);
      inventory.addItems(data.items);
    });
  })
</script>

<template>
  <NuxtPage />
</template>
