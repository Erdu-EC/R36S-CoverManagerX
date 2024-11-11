import {invoke} from "@tauri-apps/api/core";
import type {IEmulatorData} from "~/data/models/IEmulatorData";
import type {IRomData} from "~/data/models/IRomData";
import {RomData} from "~/data/models/IRomData";
import type {IGameListEntry} from "~/data/models/IGameListEntry";

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
        return invoke<IRomData[]>('filesystem_get_roms', {
            dir: dirPath,
            extensions
        }).then(roms => roms.map(rom => new RomData(rom)));
    },
    getGameList: async (dirPath: string) => {
        return invoke<IGameListEntry[]>('filesystem_get_game_list', {
            dir: dirPath
        });
    },
    openDirectoryDialog: async (onlyIfExists: boolean = true) => {
        return invoke<string | null>('filesystem_open_directory_dialog', {
            exists: onlyIfExists
        });
    }
}
