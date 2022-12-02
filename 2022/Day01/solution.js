import { TextLineStream } from "https://deno.land/std@0.167.0/streams/text_line_stream.ts";

const input = new URL("input", import.meta.url);
const lines = (await fetch(input)).body
  .pipeThrough(new TextDecoderStream())
  .pipeThrough(new TextLineStream());

const caloriesPerElf = [];
let currentCalories = 0;
for await (const line of lines) {
  const isEmptyLine = line.trim() === "";
  if (isEmptyLine) {
    caloriesPerElf.push(currentCalories);
    currentCalories = 0;
  } else {
    const calories = parseInt(line, 10);
    if (Number.isNaN(calories)) throw new Error("aaa");
    currentCalories += calories;
  }
}

const fattestElves = caloriesPerElf
  .sort((a, b) => b - a)
  .slice(0, 3);
console.log({ fattestElves });
const total = fattestElves
  .reduce((total, calories) => total + calories, 0);
console.log({ total });
