import { Artifact } from "~/classes/Artifact";
import type { StoreNotifyItem } from "~/types/types";

export const useInventoryStore = defineStore("inventoryStore", () => {
  const resources = useResources();

  const artifacts = ref<Artifact[]>([]);

  async function addArtifact(item: StoreNotifyItem) {
    const artifact = await Artifact.createFromReliquaryData(
      item,
      resources.value,
    );
    if (artifact === undefined) {
      return console.error("Couldn't create artifact from item");
    }
    artifacts.value = artifacts.value.concat(artifact);
  }

  function addItem(item: StoreNotifyItem) {
    if (item.equip?.weapon) {
      //addWeapon(item);
    } else if (item.equip?.reliq) {
      addArtifact(item);
    } else if (item.material) {
      // Add material
    } else {
      // Add other
    }
  }

  async function addItems(items: StoreNotifyItem[]) {
    console.log("Adding items", items);
    await Promise.all(items.map((item) => addItem(item)));
    console.log(artifacts.value);
  }

  return { artifacts, addItem, addItems };
});
