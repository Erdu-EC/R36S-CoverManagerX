<script setup lang="ts">

</script>

<template>
  <div>
    <header>
      <nav class="flex items-center gap-3 px-5 py-2 bg-[var(--p-primary-color)] border-b shadow-lg">
        <Icon v-tooltip="'Regresar a la selección de consola'" value="arrow_back" class="cursor-pointer"/>

        <img src="~/assets/img/r36s.png" alt="Logo R36S" class="h-10 aspect-[69/98]"/>

        <h1 class="font-bold">R36S: Gestor de portadas de juegos</h1>
      </nav>
    </header>

    <main class="p-5">
      <slot/>
    </main>
  </div>
</template>

<style scoped>

</style>