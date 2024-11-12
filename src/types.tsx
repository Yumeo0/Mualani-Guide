export type DeviceInfo = {
  index: number;
  name: string;
  description?: string;
};

export type StoreNotifyItem = {
  id: string;
  guid: string;
  material?: {
    count: string;
  }
  equip?: {
    locked?: boolean;
    weapon?: {
      level: number;
      affixMap?: {
        [key: string]: number;
      }
    }
    reliq?: {
      level: number;
      mainProp: number;
      appendPropIdList: number[];
    }
  }
};

export type StoreNotify = {
  Limit: string;
  StoreType: string;
  items: StoreNotifyItem[];
};