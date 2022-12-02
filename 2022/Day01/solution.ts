import { getInputLines } from "../input.ts";

const caloriesPerElf = [];
let currentCalories = 0;
for await (const line of getInputLines()) {
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
