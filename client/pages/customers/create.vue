<template>
  <div class="flex flex-col justify-center items-center">
    <div class="container mx-auto bg-white mt-10 rounded px-4">
      <div class="xl:w-full border-b border-gray-300 py-5">
        <div class="flex w-11/12 mx-auto xl:w-full xl:mx-0 items-center">
          <p class="text-lg text-gray-800 font-bold">Customer Information</p>
          <div class="ml-2 cursor-pointer text-gray-600">
            <img src="https://tuk-cdn.s3.amazonaws.com/can-uploader/simple_form-svg4.svg" alt="info">
            <img class="hidden" src="https://tuk-cdn.s3.amazonaws.com/can-uploader/simple_form-svg4dark.svg" alt="info">
          </div>
        </div>
      </div>
      <div class="mx-auto pt-4">
        <div class="container mx-auto">
          <form class="my-6 w-11/12 mx-auto xl:w-full xl:mx-0">
            <div class="xl:w-1/4 lg:w-1/2 md:w-1/2 flex flex-col mb-6">
              <label for="FirstName" class="pb-2 text-sm font-bold text-gray-800">First Name</label>
              <input tabindex="0" type="text" id="FirstName" name="firstName" v-model="firstName" required class="border border-gray-300 pl-3 py-3 shadow-sm bg-transparent rounded text-sm focus:outline-none focus:border-indigo-700 placeholder-gray-500 text-gray-600" placeholder="" />
            </div>
            <div class="xl:w-1/4 lg:w-1/2 md:w-1/2 flex flex-col mb-6">
              <label for="LastName" class="pb-2 text-sm font-bold text-gray-800">Last Name</label>
              <input tabindex="0" type="text" id="LastName" name="lastName" v-model="lastName" required class="border border-gray-300 pl-3 py-3 shadow-sm bg-transparent rounded text-sm focus:outline-none focus:border-indigo-700 placeholder-gray-500 text-gray-600" placeholder="" />
            </div>
            <div class="xl:w-1/4 lg:w-1/2 md:w-1/2 flex flex-col mb-6">
              <label for="PhoneNumber" class="pb-2 text-sm font-bold text-gray-800">Phone Number</label>
              <input tabindex="0" type="text" id="PhoneNumber" name="phoneNumber" v-model="phone" required class="border border-gray-300 pl-3 py-3 shadow-sm bg-transparent rounded text-sm focus:outline-none focus:border-indigo-700 placeholder-gray-500 text-gray-600" placeholder="" />
            </div>
            <div class="xl:w-1/4 lg:w-1/2 md:w-1/2 flex flex-col mb-6">
              <label for="Address" class="pb-2 text-sm font-bold text-gray-800">Address</label>
              <input tabindex="0" type="text" id="Address" name="address" v-model="address" required class="border border-gray-300 pl-3 py-3 shadow-sm bg-transparent rounded text-sm focus:outline-none focus:border-indigo-700 placeholder-gray-500 text-gray-600" placeholder="" />
            </div>
            <div class="xl:w-1/4 lg:w-1/2 md:w-1/2 flex flex-col mb-6">
              <label for="City" class="pb-2 text-sm font-bold text-gray-800">City</label>
              <input tabindex="0" type="text" id="City" name="city" v-model="city" required class="border border-gray-300 pl-3 py-3 shadow-sm bg-transparent rounded text-sm focus:outline-none focus:border-indigo-700 placeholder-gray-500 text-gray-600" placeholder="" />
            </div>
          </form>
        </div>
      </div>
    </div>
    <div class="container mx-auto w-11/12 xl:w-full">
      <div class="w-full py-4 sm:px-0 bg-white flex justify-end font-medium">
        <button role="button" @click="navigateTo('/customers')" aria-label="cancel form" class="bg-gray-200 focus:outline-none transition duration-150 ease-in-out hover:bg-gray-300 rounded text-indigo-600 px-6 py-2 text-sm mr-4 focus:ring-2 focus:ring-offset-2 focus:ring-gray-700">Back</button>
        <button role="button" @click="createItem" aria-label="Save form" class="focus:ring-2 focus:ring-offset-2 focus:ring-indigo-700 bg-indigo-700 focus:outline-none transition duration-150 ease-in-out hover:bg-indigo-600 rounded text-white px-8 py-2 text-sm" type="submit">Create</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import axios from "axios";
import {navigateTo} from "#app";

const config = useRuntimeConfig();
const firstName = ref(null);
const lastName = ref(null);
const phone = ref(null);
const address = ref(null);
const city = ref(null);

const createItem = async () => {
  const data = {
    firstName: firstName.value,
    lastName: lastName.value,
    phone: phone.value,
    address: address.value,
    city: city.value
  };

  const res = await axios.post(`${config.public.api}api/customer`, data);
  if (res.status === 200) {
    navigateTo('/customers');
  }
};
</script>
