<template>
    <nav class="p-3 flex flex-col justify-between w-[240px] h-screen">
        <div class="flex flex-col">
            <div class="flex items-center pb-3 border-b border-white/10 mb-5">
                <div class="p-3 rounded-xl border border-white/10">
                    <img src="/summerdb.png" width="25" alt="SummerDB">
                </div>
                <strong class="text-white text-xl ml-3">SummerDB</strong>
            </div>
            <ul class="flex flex-col">
                <AppLink icon="uil:github" text="Dashboard" link="/" />
                <AppLink icon="mynaui:users" text="User" link="/users" />
                <AppLink icon="material-symbols:person-add-outline" text="Create User" link="/users/create" />
            </ul>
        </div>
        <nuxt-link to="/setup" class="link">Setup</nuxt-link>
    </nav>
</template>