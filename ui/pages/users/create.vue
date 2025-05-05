<template>
    <section>
        <form @submit.prevent="store" method="POST">
            <div v-if="user" class="max-w-[400px] w-full m-auto mt-6">
                <FormInput type="text" v-model="name" placeholder="Name" required />
                <FormInput type="email" v-model="email" placeholder="Email" required />
                <FormInput type="password" v-model="password" placeholder="Password" required />
                <button type="submit" class="py-2 px-3 rounded-md bg-black text-white">Create</button>
            </div>
        </form>
    </section>
</template>

<script setup>
const user = ref(null);

const name = ref('');
const email = ref('');
const password = ref('');


const { data } = await useFetch('/api/users');
user.value = data.value;


async function store() {
    try {
        const data = await $fetch('/api/create/user', {
            method: "POST",
            body: {
                name: name.value,
                email: email.value,
                password: password.value
            }
        });
        reset();
        console.log(data);
    } catch (e) {
        console.log(e);
    }
}


function reset() {
    name.value = "";
    email.value = "";
    password.value = "";
}
</script>