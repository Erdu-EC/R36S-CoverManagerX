import {invoke} from "@tauri-apps/api/core";

export const useTauriCommands = () => {
    return {
        fileSystemCommands
    }
}

const fileSystemCommands = {
    getEasyRomsDevicePath: async () => {
        return invoke<string | null>("filesystem_get_easyroms_device_path");
    }
}
