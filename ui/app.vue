<template>
  <main class="h-screen">
    <NuxtLoadingIndicator />
    <section class="flex h-full w-full">
      <AppNavbar />
      <div class="flex-1 overflow-y-auto">
        <NuxtPage />
      </div>
    </section>
  </main>
</template>
