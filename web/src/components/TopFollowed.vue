<script setup lang="ts">
import { ref, watch } from "vue";

const get_data = async (num) => {
  if (num === 0 || num === "" || isNaN(num)) {
    num = 50;
  } else if (num > 1000) {
    num = 1000;
  }

  const result = await fetch(`${import.meta.env.VITE_API_URL || ""}/following/top/${num}`, {
    method: "GET",
  });

  return await result.json();
};

const num_followed = ref("10");
const followed = ref(await get_data(10));

watch(num_followed, async (num) => (followed.value = await get_data(num)));
</script>

<template>
  <div class="col">
    <h3 class="text-center">Top <input v-model="num_followed" size="3" /> following</h3>

    <table class="table">
      <thead>
        <tr>
          <th>#</th>
          <th>User</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="user in followed" :key="user[0]">
          <td>{{ user[1] }}</td>
          <td>
            <a :href="`https://github.com/${user[0]}`" target="_blank">{{ user[0] }}</a>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
