import type {
  ManualTextMap,
  ReliquaryAffixData,
  ReliquaryData,
  ReliquaryMainPropData,
  TextMap,
} from "~/types/ExcelConfigDataTypes";

export interface Resources {
  artifactData: ReliquaryData[];
  artifactMstatData: ReliquaryMainPropData[];
  artifactSstatData: ReliquaryAffixData[];
  manualTextMap: ManualTextMap[];
  textMap: TextMap | null;
}

export const useResources = () =>
  useState<Resources>("resources", () => {
    return {
      artifactData: [],
      artifactMstatData: [],
      artifactSstatData: [],
      manualTextMap: [],
      textMap: null,
    };
  });
