<template>
  <main class="h-screen overflow-hidden">
    <NuxtLoadingIndicator />
    <section class="flex h-full w-full p-2">
      <AppNavbar />
      <div class="flex-1 overflow-y-auto p-3 bg-primary rounded-xl">
        <NuxtPage />
      </div>
    </section>
  </main>
</template>
