import { TextLineStream } from "https://deno.land/std@0.167.0/streams/text_line_stream.ts";

const input = new URL("input", import.meta.url);
const lines = (await fetch(input)).body
  .pipeThrough(new TextDecoderStream())
  .pipeThrough(new TextLineStream());

let maxCalories = -Infinity;
let currentCalories = 0;
for await (const line of lines) {
  const isEmptyLine = line.trim() === "";
  if (isEmptyLine) {
    maxCalories = Math.max(maxCalories, currentCalories);
    currentCalories = 0;
  } else {
    const calories = parseInt(line, 10);
    if (Number.isNaN(calories)) throw new Error("aaa");
    currentCalories += calories;
  }
}

console.log({ maxCalories });
