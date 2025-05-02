<template>
    <section>
        <div v-if="users.length" class="max-w-2xl w-full m-auto mt-6">
            <div class="mb-4 bg-black p-3" v-for="(user, index) in users" :key="index">
                <div class="flex justify-between p-2 bg-gray-200">
                    <span>User:</span>
                    <span>{{ user.name }}</span>
                </div>
                <div class="flex justify-between p-2 bg-white">
                    <span>Age:</span>
                    <span>{{ user.age }}</span>
                </div>
                <div class="flex justify-between p-2 bg-gray-200">
                    <span>Address:</span>
                    <span>{{ user.street }}, {{ user.postal_code }}</span>
                </div>
            </div>
        </div>
        <div v-else>
            Loading...
        </div>
    </section>
</template>

<script setup>
const users = ref([]);
const { data } = await useFetch('/api/users');
users.value = data.value;
</script>