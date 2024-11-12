import { Resources } from "../hooks/useResourcesStore";
import { StoreNotifyItem } from "../types";

type Substat = {
    name: string;
    name_readable: string;
    value: number;
}

export class Artifact {
    id: string;
    name: string;
    description: string;
    level: number;
    rarity: number;
    mainProp: string;
    substats: Substat[];

    private constructor(id: string, name: string, description: string, level: number, rarity: number, mainProp: string, substats: Substat[]) {
        this.id = id;
        this.name = name;
        this.description = description;
        this.level = level;
        this.rarity = rarity;
        this.mainProp = mainProp;
        this.substats = substats;
    }

    static async createFromReliquaryData(data: StoreNotifyItem, resources: Resources) {
        if (resources.textMap === null) return console.error("Couldn't find text map");
        const artifactType = resources.artifactData.find((a) => a.id === parseInt(data.id));
        if (artifactType === undefined) return console.error("Couldn't find artifact type");
        const mainStatData = resources.artifactMstatData.find((a) => a.id === data?.equip?.reliq?.mainProp);
        if (mainStatData === undefined) return console.error("Couldn't find main stat data");
        const mainPropHash = resources.manualTextMap.find((m) => m.textMapId === mainStatData.propType)?.textMapContentTextMapHash;
        if (mainPropHash === undefined) return console.error("Couldn't find main prop hash");
        const artifact: Artifact = {
            id: data.id,
            name: resources.textMap[artifactType.nameTextMapHash] ?? '',
            description: resources.textMap[artifactType.descTextMapHash] ?? '',
            level: data.equip?.reliq?.level ?? 0,
            rarity: artifactType.rankLevel,
            mainProp: resources.textMap[mainPropHash] ?? '',
            substats: Artifact.createSubstats(data.equip?.reliq?.appendPropIdList ?? [], resources),
        };
        return artifact;
    }

    private static createSubstats(substatRolls: number[], resources: Resources) {
        if (resources.textMap === null) {
            console.error("Couldn't find text map");
            return [];
        }
        const substats: Substat[] = [];
        for (let i = 0; i < substatRolls.length; i++) {
            const substatRoll = substatRolls[i];
            const substatData = resources.artifactSstatData.find((a) => a.id === substatRoll);
            if (substatData === undefined) continue;
            const substatHash = resources.manualTextMap.find((m) => m.textMapId === substatData.propType)?.textMapContentTextMapHash;
            if (substatHash === undefined) continue;
            const substat = substats.findIndex((s) => s.name === substatData.propType);
            if (substat === -1) {
                substats.push({
                    name: substatData.propType,
                    name_readable: resources.textMap[substatHash] ?? '',
                    value: substatData.propValue,
                });
            } else {
                substats[substat].value += substatData.propValue;
            }
        }
        return substats;
    }
}