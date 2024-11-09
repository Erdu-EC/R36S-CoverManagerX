import {invoke} from "@tauri-apps/api/core";
import type {IEmulatorData} from "~/data/models/IEmulatorData";

export const useTauriCommands = () => {
    return {
        fileSystemCommands
    }
}

const fileSystemCommands = {
    getEasyRomsDevicePath: async () => {
        return invoke<string | null>("filesystem_get_easyroms_device_path");
    },
    getEmulatorsData: async (dirPath: string) => {
        return invoke<IEmulatorData[]>('filesystem_get_emulators', {
            dir: dirPath
        });
    },
    getRoms: async (dirPath: string, extensions?: string[]) => {
        return invoke<string[]>('filesystem_get_roms', {
            dir: dirPath,
            extensions
        });
    }
}
