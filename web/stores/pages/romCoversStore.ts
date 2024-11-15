import type {RomData} from "~/data/models/IRomData";

export const useRomCoversStore = defineStore('romCoversStore', () => {
    const roms = ref<RomData[]>();
    const setRoms = (romsData: RomData[]) => roms.value = romsData;
    const clearRoms = () => roms.value = undefined;

    return {
        roms,
        setRoms,
        clearRoms,
    }
});