<script setup lang="ts">
import type {RomData} from "~/data/models/IRomData";
import type {VirtualScrollerItemOptions} from "primevue";

const route = useRoute();
const {emulator, getRomList} = useRomCoversPage(route.params.console as string);

const {data: roms, error, status, refresh} = useLazyAsyncData(`${emulator.value.name}-roms`, getRomList);
</script>

<template>
  <div class="h-full">
    <Loading v-if="status === 'pending'" :label="$t('pages.romList.loading')" class="mt-5"/>

    <ErrorCard v-else-if="status === 'error' && error" class="mt-5"
               :title="$t('pages.romList.errorTitle')"
               :error="error"
               @refresh="refresh"/>

    <Splitter v-else-if="roms" class="h-full">
      <SplitterPanel class="flex items-center justify-center" :size="60" :minSize="30">
        <VirtualScroller :items="roms" :itemSize="153" class="w-full rounded" style="height: 100%"
                         :pt-options="{mergeProps: true}" :pt="{
                          content: 'w-full'
                          }">
          <template v-slot:item="{ item, options } : {item: RomData, options: VirtualScrollerItemOptions}">
            <div v-ripple :class="['flex flex-col sm:flex-row sm:items-center p-5 gap-4 cursor-pointer', {
            'bg-surface-100 dark:bg-surface-700': options.odd,
             'border-t border-surface-200 dark:border-surface-700': options.index !== 0
          }]">
              <div class="md:w-28 relative">
                <ImagePlaceholder :src="item.imageUrl" alt="Cover"
                                  class="flex items-center size-28 aspect-square mx-auto rounded"
                                  imageClass="object-scale-down"/>
              </div>
              <div class="flex flex-row md:flex-col justify-between items-start gap-2 overflow-hidden">
                <div class="w-full">
                  <div class="text-lg font-medium mt-2" :title="item.nameString">{{ item.nameString }}</div>
                  <div class="font-medium text-surface-500 dark:text-surface-400 text-sm truncate" :title="item.path">
                    {{ item.path }}
                  </div>
                </div>
                <div class="flex items-center gap-2">
                  <div class="bg-surface-100 p-1" style="border-radius: 30px">
                    <div class="bg-surface-0 flex items-center gap-2 justify-center py-1 px-2"
                         style="border-radius: 30px; box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.04), 0 1px 2px 0 rgba(0, 0, 0, 0.06)">
                      <span class="text-surface-900 font-medium text-sm">{{ toFriendlyDataUnit(item.length) }}</span>
                    </div>
                  </div>
                  <Tag :value="'.' + item.extensionString"></Tag>
                </div>
              </div>
            </div>
          </template>
        </VirtualScroller>
      </SplitterPanel>
      <SplitterPanel class="flex items-center justify-center p-5" :size="40" :min-size="20">
        <div class="flex flex-col items-center gap-4 text-xl text-center text-pretty text-surface-500">
          Seleccione un juego para ver o modificar su imagen de portada y demás información.
        </div>
      </SplitterPanel>
    </Splitter>
  </div>
</template>

<style scoped>

</style>