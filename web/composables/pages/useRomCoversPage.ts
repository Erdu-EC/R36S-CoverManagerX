import {path} from "@tauri-apps/api";

export const useRomCoversPage = (emulatorCode: string) => {
    const appStore = useAppStore();
    const {easyRomsPath, emulatorsData} = storeToRefs(appStore);

    const romCoversStore = useRomCoversStore();
    const {setRoms, clearRoms} = romCoversStore;
    const {roms} = storeToRefs(romCoversStore);

    const emulator = computed(() =>
        emulatorsData.value?.filter(e => e.name === emulatorCode).shift()!);

    const getRomList = async () => {
        clearRoms();

        const emulatorPath = await path.join(easyRomsPath.value, emulator.value.name);

        const {fileSystemCommands} = useTauriCommands();
        const roms = await fileSystemCommands.getRoms(emulatorPath, emulator.value.formats);
        const gameList = await fileSystemCommands.getGameList(emulatorPath);

        for (const rom of roms) {
            const relativePath = "./" + rom.path.substring(emulatorPath.length + 1).replace('\\', '/');
            const game = gameList.filter((game) => game.path.replace('\\', '/') == relativePath).shift();
            if (game)
                await rom.loadGameListEntry(game, emulatorPath);
        }

        setRoms(roms);

        return roms;
    }

    return {
        emulator,
        roms,
        getRomList,
    }
}