<script setup lang="ts">
import {path} from "@tauri-apps/api";
import type {RomData} from "~/data/models/IRomData";

const route = useRoute();
const appStore = useAppStore();
const {fileSystemCommands} = useTauriCommands();
const {easyRomsPath, emulatorsData} = storeToRefs(appStore);

const code = route.params.console as string;
const emulator = computed(() => emulatorsData.value?.filter((emulator) => emulator.name === code).shift()!);
const emulatorPath = await path.join(easyRomsPath.value, emulator.value.name);

const {data: roms, error, status, refresh} = useAsyncData(`${code}-roms`, async () => {
  const roms = await fileSystemCommands.getRoms(emulatorPath, emulator.value.formats);
  const gameList = await fileSystemCommands.getGameList(emulatorPath);

  for (const rom of roms) {
    const relativePath = "./" + rom.path.substring(emulatorPath.length + 1).replace('\\', '/');
    const game = gameList.filter((game) => game.path.replace('\\', '/') == relativePath).shift();
    if (game)
      await rom.loadGameListEntry(game, emulatorPath);
  }

  return roms;
});
</script>

<template>
  <div>
    <Loading v-if="status === 'pending'" :label="$t('pages.romList.loading')"/>
    <ErrorCard v-else-if="status === 'error' && error" class="mt-5"
               :title="$t('pages.romList.errorTitle')"
               :error="error"
               @refresh="refresh"/>

    <DataView v-if="roms" :value="roms" dataKey="path">
      <template #list="slotProps">
        <div class="flex flex-col">
          <div v-for="(item, index) in slotProps.items as RomData[]" :key="index">
            <div class="flex flex-col sm:flex-row sm:items-center p-6 gap-4"
                 :class="{ 'border-t border-surface-200 dark:border-surface-700': index !== 0 }">
              <div class="md:w-28 relative">
                <ImagePlaceholder :src="item.imageUrl" alt="Cover"
                                  class="flex items-center size-28 aspect-square mx-auto rounded"
                                  imageClass="object-scale-down"/>
                <div class="absolute bg-black/70 rounded-border" style="right: 4px; bottom: 4px">

                </div>
              </div>
              <div class="flex flex-col md:flex-row justify-between md:items-center flex-1 gap-6">
                <div class="flex flex-row md:flex-col justify-between items-start gap-2">
                  <div>
                    <div class="text-lg font-medium mt-2">{{ item.nameString }}</div>
                    <span class="font-medium text-surface-500 dark:text-surface-400 text-sm">
                      {{ item.path }}
                    </span>
                  </div>
                  <div class="flex items-center gap-2">
                    <div class="bg-surface-100 p-1" style="border-radius: 30px">
                      <div class="bg-surface-0 flex items-center gap-2 justify-center py-1 px-2"
                           style="border-radius: 30px; box-shadow: 0px 1px 2px 0px rgba(0, 0, 0, 0.04), 0px 1px 2px 0px rgba(0, 0, 0, 0.06)">
                        <span class="text-surface-900 font-medium text-sm">{{ toFriendlyDataUnit(item.length) }}</span>
                      </div>
                    </div>
                    <Tag :value="'.' + item.extensionString"></Tag>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </template>
    </DataView>
  </div>
</template>

<style scoped>

</style>