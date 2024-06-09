<script setup lang="ts">
import * as d3 from "d3";
import { ref, watch } from "vue";

const get_data = async (num) => {
  if (num === 0 || num === "" || isNaN(num)) {
    num = 50;
  } else if (num > 1000) {
    num = 1000;
  }

  const result = await fetch(`${import.meta.env.VITE_API_URL || ""}/repos/top/${num}`, {
    method: "GET",
  });

  let json: [string, number][] = await result.json();

  const root = pack(d3.hierarchy({ children: json } as unknown).sum((d: any) => d[1]));
  return root.leaves();
};

const width = 700;
const height = window.innerHeight - 103; // height of header
const margin = 0;
const color = d3.scaleOrdinal(d3.schemeTableau10);
const pack = d3
  .pack()
  .size([width - margin * 2, height - margin * 2])
  .padding(3);

const num_starred = ref("50");
const leaves = ref(await get_data(50));

watch(num_starred, async (num) => (leaves.value = await get_data(num)));
</script>

<template>
  <div class="col" :style="`width: ${width}px`">
    <h3 class="text-center">Top <input v-model="num_starred" size="3" /> starred repos</h3>

    <svg
      :width="width"
      :height="height"
      :viewBox="[-margin, -margin, width, height].join(' ')"
      style="max-width: 100%; height: auto; font-size: 80%"
      text-anchor="middle"
    >
      <g v-for="leaf in leaves" :key="leaf.data[0]" :transform="`translate(${leaf.x}, ${leaf.y})`">
        <a
          :href="`https://github.com/${leaf.data[0]}`"
          style="text-decoration: none"
          target="_blank"
        >
          <title>{{ leaf.data[0] }}: {{ leaf.data[1] }}</title>
          <circle fill-opacity="0.7" :fill="color(leaf.data[1])" :r="leaf.r"></circle>
          <text :clip-path="`circle(${leaf.r})`">
            <tspan x="0" y="-0.65em">{{ leaf.data[0].split("/")[0] }}</tspan>
            <tspan x="0" y="0.35em">{{ leaf.data[0].split("/")[1] }}</tspan>
            <tspan x="0" y="1.35em" fill-opacity="0.7">{{ leaf.data[1] }}</tspan>
          </text>
        </a>
      </g>
    </svg>
  </div>
</template>
