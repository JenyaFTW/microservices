<template>
  <div class="h-full">
    <nav class="bg-white shadow dark:bg-gray-800">
      <div
        class="container flex items-center justify-center p-6 mx-auto text-gray-600 text-sm font-medium capitalize dark:text-gray-300"
      >
        <NuxtLink
          to="/"
          :class="`text-gray-800 transition-colors duration-300 transform dark:text-gray-200 mx-1.5 sm:mx-6 ${
            route.name === 'index' && 'border-b-2 border-blue-500'
          }`"
          >home</NuxtLink
        >

        <NuxtLink
          to="/customers"
          :class="`hover:text-gray-800 transition-colors duration-300 transform dark:hover:text-gray-200 hover:border-blue-500 mx-1.5 sm:mx-6 ${
            route.name === 'customers' ? 'border-b-2 border-blue-500' : ''
          }`"
          >customers</NuxtLink
        >

        <NuxtLink
          to="/orders"
          :class="`hover:text-gray-800 transition-colors duration-300 transform dark:hover:text-gray-200 hover:border-blue-500 mx-1.5 sm:mx-6 ${
            route.name === 'orders' && 'border-b-2 border-blue-500'
          }`"
          >orders</NuxtLink
        >

        <NuxtLink
          to="/warehouse"
          :class="`hover:text-gray-800 transition-colors duration-300 transform dark:hover:text-gray-200 hover:border-blue-500 mx-1.5 sm:mx-6 ${
            route.name === 'warehouse' && 'border-b-2 border-blue-500'
          }`"
          >warehouse</NuxtLink
        >
      </div>
    </nav>
    <slot />
  </div>
</template>

<script setup>
const route = useRoute();
</script>

<style>
html,
body,
#__nuxt {
  padding: 0;
  margin: 0;
  width: 100%;
  height: 100%;
}
</style>
