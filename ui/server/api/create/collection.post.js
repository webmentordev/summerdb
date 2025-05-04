export default defineEventHandler(async (event) => {
    const api = useRuntimeConfig(event).api;
    let body_content = await readBody(event);
    let result = await $fetch(`${api}/api/create/collection`, {
        method: "POST",
        body: body_content,
    });
    return result;
})