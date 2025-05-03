<template>
    <nav class="p-3 flex flex-col justify-between w-[80px] h-screen">
        <div class="flex flex-col items-center">
            <div class="flex items-center pb-3 border-b border-white/10 mb-5">
                <div class="p-3 rounded-xl border border-white/10">
                    <img src="/summerdb.png" width="25" alt="SummerDB">
                </div>
            </div>
            <ul class="flex flex-col m-auto">
                <AppLink icon="akar-icons:dashboard" text="Dashboard" link="/" />
                <AppLink icon="mynaui:users" text="User" link="/users" />
                <AppLink icon="material-symbols:person-add-outline" text="Create" link="/users/create" />
                <AppLink icon="charm:database" text="Collections" link="/collections" />
            </ul>
        </div>
        <nuxt-link to="/setup" class="link">Setup</nuxt-link>
    </nav>
</template>