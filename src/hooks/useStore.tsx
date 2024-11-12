import { useEffect, useState } from 'react';
import { StoreNotifyItem } from '../types';
import { Artifact } from '../classes/Artifact';
import useResourcesStore from './useResourcesStore';

function useStore() {
  const [artifacts, setArtifacts] = useState<Artifact[]>([]);
  const resources = useResourcesStore();

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

  async function addArtifact(item: StoreNotifyItem) {
    const artifact = await Artifact.createFromReliquaryData(item, useResourcesStore.getState());
    if (artifact === undefined) return console.error("Couldn't create artifact from item");
    setArtifacts((artifacts) => artifacts.concat(artifact));
  }

  async function addItems(items: StoreNotifyItem[]) {
    await resources.loadResources();
    items.forEach((item) => addItem(item));
  }

  useEffect(() => {
    console.log(artifacts);
  }, [artifacts]);

  return { artifacts, addItem, addItems };
}

export default useStore;