import { getInputLines } from "../input.ts";

enum Shape {
  Rock = 1,
  Paper = 2,
  Scissors = 3,
}

function parseOpponentPick(pick: string): Shape {
  switch (pick) {
    case "A":
      return Shape.Rock;
    case "B":
      return Shape.Paper;
    case "C":
      return Shape.Scissors;
    default:
      throw new Error("stupid elf");
  }
}

function parseMyPick(pick: string): Shape {
  switch (pick) {
    case "X":
      return Shape.Rock;
    case "Y":
      return Shape.Paper;
    case "Z":
      return Shape.Scissors;
    default:
      throw new Error("i don't know what to do!");
  }
}

enum Outcome {
  Win = 6,
  Draw = 3,
  Loss = 0,
}

function getOutcome(a: Shape, b: Shape): Outcome {
  switch (a) {
    case Shape.Rock:
      return b === Shape.Rock
        ? Outcome.Draw
        : b === Shape.Paper
        ? Outcome.Loss
        : Outcome.Win;
    case Shape.Paper:
      return b === Shape.Rock
        ? Outcome.Win
        : b === Shape.Paper
        ? Outcome.Draw
        : Outcome.Loss;
    case Shape.Scissors:
      return b === Shape.Rock
        ? Outcome.Loss
        : b === Shape.Paper
        ? Outcome.Win
        : Outcome.Draw;
  }
}

function score(me: Shape, opponent: Shape): number {
  const outcome = getOutcome(me, opponent);
  // Abusing that the enum values are defined as their score
  return me + outcome;
}

let totalScore = 0;
for await (const line of getInputLines()) {
  if (!line.trim()) continue; // final newline

  const [opponentPick, myPick] = line.split(" ");
  const opponentShape = parseOpponentPick(opponentPick);
  const myShape = parseMyPick(myPick);
  totalScore += score(myShape, opponentShape);
}

console.log({ totalScore });
