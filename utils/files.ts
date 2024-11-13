import * as tauriPath from "@tauri-apps/api/path";
import { readTextFile } from "@tauri-apps/plugin-fs";

export async function readData<T>(filePath: string) {
  try {
    const resource = await tauriPath.resolveResource(filePath);
    const data = await readTextFile(resource);
    return JSON.parse(data) as T;
  } catch (error) {
    console.error(`Failed to load ${filePath}:`, error);
  }
}
