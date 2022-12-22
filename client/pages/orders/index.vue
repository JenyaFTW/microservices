<template>
  <div class="index">
    <div class="w-full sm:px-6">
      <div
        class="px-4 md:px-10 py-4 md:py-7 rounded-tl-lg rounded-tr-lg"
      >
        <div class="sm:flex items-center justify-between">
          <p
            tabindex="0"
            class="focus:outline-none text-base sm:text-lg md:text-xl lg:text-2xl font-bold leading-normal text-gray-800 flex flex-col space-y-2"
          >
            <span>Orders</span>
            <span v-if="orders.length > 0" class="text-sm text-gray-600 font-medium">Total: {{ orders.length }}</span>
          </p>
          <div>
            <button
              class="focus:ring-2 focus:ring-offset-2 focus:ring-indigo-600 inline-flex sm:ml-3 mt-4 sm:mt-0 items-start justify-start px-6 py-3 bg-indigo-700 hover:bg-indigo-600 focus:outline-none rounded"
            >
              <nuxt-link to="/orders/create" class="text-sm font-medium leading-none text-white">
                New order
              </nuxt-link>
            </button>
          </div>
        </div>
      </div>
      <div
        v-if="orders.length > 0"
        class="px-4 md:px-10 pb-5 overflow-y-auto"
      >
        <table class="w-full whitespace-nowrap">
          <thead>
            <tr
              tabindex="0"
              class="focus:outline-none h-16 w-full text-sm leading-none text-gray-800"
            >
              <th class="font-normal text-left pl-4">Order ID</th>
              <th class="font-normal text-left pl-12">Client ID</th>
              <th class="font-normal text-left pl-12"># of items</th>
            </tr>
          </thead>
          <tbody class="w-full">
            <tr
              v-for="(order, idx) in orders"
              :id="idx"
              @click="navigateTo(`/orders/${order.id}`)"
              tabindex="0"
              class="focus:outline-none h-20 text-sm leading-none text-gray-800 bg-white hover:bg-gray-100 border-b border-t border-gray-100"
            >
              <td class="pl-4 cursor-pointer text-gray-800 font-semibold">
                {{ order.id }}
              </td>
              <td class="pl-12 cursor-pointer text-gray-800 font-semibold">
                {{ order.clientId }}
              </td>
              <td class="pl-12 cursor-pointer text-gray-800 font-semibold">
                {{ order.itemId.length }}
              </td>
              <td>
                <button v-on:click.stop @click="() => deleteOrder(order.id)" class="text-red-500 p-2 border-transparent border bg-gray-100 hover:bg-gray-200 cursor-pointer rounded focus:outline-none focus:border-gray-800 focus:shadow-outline-gray" aria-label="delete table" role="button">
                  <img src="https://tuk-cdn.s3.amazonaws.com/can-uploader/compact_table_with_actions_and_select-svg5.svg" alt="delete">
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup>
import {navigateTo} from "#app";
import axios from 'axios';

const config = useRuntimeConfig();
const orders = ref([]);

const fetchOrders = async () => {
  const res = await axios.get(`${config.public.api}api/orders`);
  orders.value = res.data;
};

const deleteOrder = async id => {
  const res = await axios.delete(`${config.public.api}api/orders/delete/${id}`);
  if (res.status === 200) {
    await fetchOrders();
  }
};

onMounted(async () => {
  await fetchOrders();
});
</script>

<style scoped>
.index {
  display: flex;
  flex-direction: column;
}
</style>
