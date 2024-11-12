import { create } from 'zustand';
import { readTextFile } from '@tauri-apps/plugin-fs';
import * as tauriPath from '@tauri-apps/api/path';
import {
    ManualTextMap,
    ReliquaryAffixData,
    ReliquaryData,
    ReliquaryMainPropData,
    TextMap
} from '../types/ExcelConfigDataTypes';

export interface Resources {
    artifactData: ReliquaryData[];
    artifactMstatData: ReliquaryMainPropData[];
    artifactSstatData: ReliquaryAffixData[];
    manualTextMap: ManualTextMap[];
    textMap: TextMap | null;
    loadResources: () => Promise<void>;
}

const useResourcesStore = create<Resources>((set) => ({
    artifactData: [],
    artifactMstatData: [],
    artifactSstatData: [],
    manualTextMap: [],
    textMap: null,

    loadResources: async () => {
        async function readData<T>(path: string, setData: (data: T) => void) {
            try {
                const filePath = await tauriPath.resolveResource(path);
                const data = await readTextFile(filePath);
                const json = JSON.parse(data) as T;
                setData(json);
            } catch (error) {
                console.error(`Failed to load ${path}:`, error);
            }
        }

        await readData<ReliquaryData[]>('resources/gi/data/ReliquaryExcelConfigData.json',
            (data) => set({ artifactData: data }));

        await readData<ReliquaryMainPropData[]>('resources/gi/data/ReliquaryMainPropExcelConfigData.json',
            (data) => set({ artifactMstatData: data }));

        await readData<ReliquaryAffixData[]>('resources/gi/data/ReliquaryAffixExcelConfigData.json',
            (data) => set({ artifactSstatData: data }));

        await readData<ManualTextMap[]>('resources/gi/data/ManualTextMapConfigData.json',
            (data) => set({ manualTextMap: data }));

        await readData<TextMap>('resources/gi/data/TextMapEN.json',
            (data) => set({ textMap: data }));
    },
}));

export default useResourcesStore;
