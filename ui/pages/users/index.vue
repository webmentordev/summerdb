<template>
    <section>
        <div v-if="users.length" class="max-w-2xl w-full m-auto mt-6">
            <div class="mb-4 bg-black p-3" v-for="(user, index) in users" :key="index">
                <div class="flex justify-between p-2 bg-gray-200">
                    <span>Name:</span>
                    <span>{{ user.name }}</span>
                </div>
                <div class="flex justify-between p-2 bg-white">
                    <span>Email:</span>
                    <span>{{ user.email }}</span>
                </div>
                <div class="flex justify-between p-2 bg-gray-200">
                    <span>Password:</span>
                    <span>{{ user.password }}</span>
                </div>
                <div class="flex justify-between p-2 bg-white">
                    <span>Created:</span>
                    <span>{{ user.created_at }}</span>
                </div>
                <div class="flex justify-between p-2 bg-gray-200">
                    <span>Updated:</span>
                    <span>{{ user.updated_at }}</span>
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