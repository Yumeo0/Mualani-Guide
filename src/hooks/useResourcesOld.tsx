import { useEffect, useState } from "react";
import { ManualTextMap, ReliquaryAffixData, ReliquaryData, ReliquaryMainPropData, TextMap } from "../types/ExcelConfigDataTypes";
import { readTextFile } from "@tauri-apps/plugin-fs";
import * as tauriPath from "@tauri-apps/api/path";

export interface Resources {
    artifactData: ReliquaryData[];
    artifactMstatData: ReliquaryMainPropData[];
    artifactSstatData: ReliquaryAffixData[];
    manualTextMap: ManualTextMap[];
    textMap: TextMap;
    loadResources: () => Promise<void>;
}

function useResources() {
    const [artifactData, setArtifactData] = useState<ReliquaryData[]>([]);
    const [artifactMstatData, setArtifactMstatData] = useState<ReliquaryMainPropData[]>([]);
    const [artifactSstatData, setArtifactSstatData] = useState<ReliquaryAffixData[]>([]);
    const [manualTextMap, setManualTextMap] = useState<ManualTextMap[]>([]);
    const [textMap, setTextMap] = useState<TextMap | null>(null);

    async function readData<T>(path: string, setData: (data: T) => void) {
        return readTextFile(await tauriPath.resolveResource(path))
            .then(data => {
                let json = JSON.parse(data) as T;
                return json;
            })
            .then(setData)
            .catch(console.error);
    }

    async function loadResources() {
        await readData<ReliquaryData[]>("resources/gi/data/ReliquaryExcelConfigData.json", setArtifactData);
        await readData<ReliquaryMainPropData[]>("resources/gi/data/ReliquaryMainPropExcelConfigData.json", setArtifactMstatData);
        await readData<ReliquaryAffixData[]>("resources/gi/data/ReliquaryAffixExcelConfigData.json", setArtifactSstatData);
        await readData<ManualTextMap[]>("resources/gi/data/ManualTextMapConfigData.json", setManualTextMap);
        await readData<TextMap>("resources/gi/data/TextMapEN.json", setTextMap);
    }

    useEffect(() => {
        loadResources();
    }, []);

    return { artifactData, artifactMstatData, artifactSstatData, manualTextMap, textMap, loadResources } as Resources;
}

export default useResources;