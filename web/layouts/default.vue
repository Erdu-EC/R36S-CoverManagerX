<script setup lang="ts">
const route = useRoute();
const isIndexPage = computed(() => route.path === '/');
</script>

<template>
  <div class="h-dvh flex flex-col">
    <header>
      <nav class="flex items-center gap-3 px-5 py-2 bg-[var(--p-primary-color)] border-b shadow-lg">
        <NuxtLinkLocale to="/">
          <Icon v-if="!isIndexPage" v-tooltip="'Regresar a la selección de consola'" value="arrow_back"
                class="cursor-pointer"/>
        </NuxtLinkLocale>

        <img src="~/assets/img/r36s.png" alt="Logo R36S" class="h-10 aspect-[69/98]"/>

        <h1 class="font-bold">R36S: Gestor de portadas de juegos</h1>
      </nav>
    </header>

    <main class="p-5 grow overflow-auto">
      <slot/>
    </main>
  </div>
</template>

<style scoped>

</style>