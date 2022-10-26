<template>
  <div class="index">
    Customers:
    {{ data }}
  </div>
</template>

<script setup>
import { useAuthStore } from '~~/stores/auth';

const authStore = useAuthStore();
const { data, pending, error, refresh } = await useAsyncData(
  'customers',
  () => $fetch('/api/customers')
);

</script>

<style scoped>
.index {
  display: flex;
  flex-direction: column;
}
</style>
