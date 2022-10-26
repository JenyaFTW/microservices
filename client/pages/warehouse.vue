<template>
  <div class="index">
    Warehouse:
    {{ data }}
  </div>
</template>

<script setup>
import { useAuthStore } from '~~/stores/auth';

const authStore = useAuthStore();
const { data, pending, error, refresh } = await useAsyncData(
  'warehouse',
  () => $fetch('/api/warehouse')
);

</script>

<style scoped>
.index {
  display: flex;
  flex-direction: column;
}
</style>
