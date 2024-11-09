import type {IEmulatorData} from "~/data/models/IEmulatorData";

export const useAppStore = defineStore('appStore', () => {
    const _easyRomsPath = ref<string>('');
    const easyRomsPath = computed(() => _easyRomsPath.value);

    const _emulatorsData = ref<IEmulatorData[]>([]);
    const emulatorsData = computed(() => _emulatorsData.value);

    const setEasyRomsPath = (path: string) => {
        _easyRomsPath.value = path;
    }

    const fetchEmulators = async () => {
        const {fileSystemCommands} = useTauriCommands();
        _emulatorsData.value = await fileSystemCommands.getEmulatorsData(_easyRomsPath.value);
    }

    return {
        _easyRomsPath,
        easyRomsPath,
        setEasyRomsPath,
        _emulatorsData,
        emulatorsData,
        fetchEmulators,
    }
})