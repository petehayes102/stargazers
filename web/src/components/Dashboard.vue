<script setup lang="ts">
import * as d3 from "d3";

const result = await fetch(`${import.meta.env.VITE_API_URL || ""}/repos/top/50`, {
  method: "GET",
});

let json: [string, number][] = await result.json();

// Specify the dimensions of the chart.
const width = 800;
const height = window.innerHeight - 72; // 72 = height of header
const margin = 1; // to avoid clipping the root circle stroke

// Create a categorical color scale.
const color = d3.scaleOrdinal(d3.schemeTableau10);

// Create the pack layout.
const pack = d3
  .pack()
  .size([width - margin * 2, height - margin * 2])
  .padding(3);

// Compute the hierarchy from the (flat) data; expose the values
// for each node; lastly apply the pack layout.
const root = pack(d3.hierarchy({ children: json } as unknown).sum((d: any) => d[1]));

// Create the SVG container.
const svg = d3.create("svg");

// Place each (leaf) node according to the layout’s x and y values.
const node = svg
  .append("g")
  .selectAll()
  .data(root.leaves())
  .join("g")
  .attr("transform", (d: any) => `translate(${d.x},${d.y})`);

node.append("title").text((d: any) => `${d.data[0]}\n${d.data[1]}`);

node
  .append("circle")
  .attr("fill-opacity", 0.7)
  .attr("fill", (d: any) => color(d.data[1]))
  .attr("r", (d: any) => d.r);

node
  .append("text")
  .attr("clip-path", (d: any) => `circle(${d.r})`)
  .selectAll()
  .data((d: any) => {
    return [...d.data[0].split("/"), d.data[1]];
  })
  .join("tspan")
  .attr("x", 0)
  .attr("y", (d: any, i: any, nodes: any) => `${i - nodes.length / 2 + 0.35}em`)
  .text((d: any) => d);

const svghtml = svg.html();
</script>

<template>
  <svg
    :width="width"
    :height="height"
    :viewBox="[-margin, -margin, width, height].join(' ')"
    style="max-width: 100%; height: auto; font-size: 80%"
    text-anchor="middle"
    v-html="svghtml"
  ></svg>
</template>
