export type ReliquaryData = {
    equipType: string;
    showPic: string;
    rankLevel: number;
    mainPropDepotId: number;
    appendPropDepotId: number;
    addPropLevels: [];
    baseConvExp: number;
    maxLevel: number;
    destroyReturnMaterial: [];
    destroyReturnMaterialCount: [];
    id: number;
    nameTextMapHash: number;
    descTextMapHash: number;
    icon: string;
    itemType: string;
    weight: number;
    rank: number;
    gadgetId: number;
}

export type ReliquaryMainPropData = {
    id: number;
    propDepotId: number;
    propType: string;
    affixName: string;
}

export type ReliquaryAffixData = {
    id: number;
    depotId: number;
    groupId: number;
    propType: string;
    propValue: number;
}

export type TextMap = {
    [id: string]: string;
}

export type ManualTextMap = {
    textMapId: string;
    textMapContentTextMapHash: number;
    paramTypes: string[];
}