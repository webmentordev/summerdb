<template>
    <nav class="bg-black p-3 flex flex-col justify-between font-semibold w-[150px] h-screen">
        <ul class="flex flex-col">
            <nuxt-link to="/" class="link">Home</nuxt-link>
            <nuxt-link to="/users" class="link">Users</nuxt-link>
            <nuxt-link to="/users/create" class="link">Create</nuxt-link>
        </ul>
        <nuxt-link to="/setup" class="link">Setup</nuxt-link>
    </nav>
</template>